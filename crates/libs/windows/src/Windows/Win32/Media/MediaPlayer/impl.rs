#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeed {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeed";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedImpl, const OFFSET: isize>() -> IFeedVtbl {
        unsafe extern "system" fn Xml<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(count, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUrl(&*(&feedurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId(::core::mem::transmute_copy(&feedguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(&*(&newparentpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWriteTime(::core::mem::transmute_copy(&lastwrite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncSetting<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncSetting(::core::mem::transmute_copy(&syncsetting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSyncSetting(syncsetting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interval<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval(::core::mem::transmute_copy(&minutes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterval(minutes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime(::core::mem::transmute_copy(&lastdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalEnclosurePath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(itemid, ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&title)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homepage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&homepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image(::core::mem::transmute_copy(&imageurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBuildDate(::core::mem::transmute_copy(&lastbuilddate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate(::core::mem::transmute_copy(&lastpopulatedate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ttl(::core::mem::transmute_copy(&ttl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language(::core::mem::transmute_copy(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyright: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copyright(::core::mem::transmute_copy(&copyright)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxItemCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxItemCount(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically(::core::mem::transmute_copy(&downloadenclosuresautomatically)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDownloadEnclosuresAutomatically(downloadenclosuresautomatically) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError(::core::mem::transmute_copy(&error)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Merge(&*(&feedxml as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&feedurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsList(::core::mem::transmute_copy(&islist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAllItemsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(scope, mask, ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadItemCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeed>,
            ::windows::core::GetTrustLevel,
            Xml::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Rename::<Impl, OFFSET>,
            Url::<Impl, OFFSET>,
            SetUrl::<Impl, OFFSET>,
            LocalId::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            LastWriteTime::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Download::<Impl, OFFSET>,
            AsyncDownload::<Impl, OFFSET>,
            CancelAsyncDownload::<Impl, OFFSET>,
            SyncSetting::<Impl, OFFSET>,
            SetSyncSetting::<Impl, OFFSET>,
            Interval::<Impl, OFFSET>,
            SetInterval::<Impl, OFFSET>,
            LastDownloadTime::<Impl, OFFSET>,
            LocalEnclosurePath::<Impl, OFFSET>,
            Items::<Impl, OFFSET>,
            GetItem::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            Link::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            LastBuildDate::<Impl, OFFSET>,
            PubDate::<Impl, OFFSET>,
            Ttl::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            Copyright::<Impl, OFFSET>,
            MaxItemCount::<Impl, OFFSET>,
            SetMaxItemCount::<Impl, OFFSET>,
            DownloadEnclosuresAutomatically::<Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically::<Impl, OFFSET>,
            DownloadStatus::<Impl, OFFSET>,
            LastDownloadError::<Impl, OFFSET>,
            Merge::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            IsList::<Impl, OFFSET>,
            MarkAllItemsRead::<Impl, OFFSET>,
            GetWatcher::<Impl, OFFSET>,
            UnreadItemCount::<Impl, OFFSET>,
            ItemCount::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeed2Impl: Sized + IFeedImpl + IDispatchImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeed2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeed2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeed2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2Impl, const OFFSET: isize>() -> IFeed2Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByEffectiveId(itemeffectiveid, ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastItemDownloadTime(::core::mem::transmute_copy(&lastitemdownloadtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username(::core::mem::transmute_copy(&username)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password(::core::mem::transmute_copy(&password)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCredentials(&*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&password as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCredentials<Impl: IFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFeed2>, ::windows::core::GetTrustLevel, GetItemByEffectiveId::<Impl, OFFSET>, LastItemDownloadTime::<Impl, OFFSET>, Username::<Impl, OFFSET>, Password::<Impl, OFFSET>, SetCredentials::<Impl, OFFSET>, ClearCredentials::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedEnclosure {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedEnclosure";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedEnclosureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosureImpl, const OFFSET: isize>() -> IFeedEnclosureVtbl {
        unsafe extern "system" fn Url<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url(::core::mem::transmute_copy(&enclosureurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&mimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError(::core::mem::transmute_copy(&error)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPath(::core::mem::transmute_copy(&localpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&enclosureurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMimeType(::core::mem::transmute_copy(&mimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFile<Impl: IFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadmimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enclosurefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFile(
                &*(&downloadurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&downloadfilepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&downloadmimetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&enclosurefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedEnclosure>,
            ::windows::core::GetTrustLevel,
            Url::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Length::<Impl, OFFSET>,
            AsyncDownload::<Impl, OFFSET>,
            CancelAsyncDownload::<Impl, OFFSET>,
            DownloadStatus::<Impl, OFFSET>,
            LastDownloadError::<Impl, OFFSET>,
            LocalPath::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            DownloadMimeType::<Impl, OFFSET>,
            RemoveFile::<Impl, OFFSET>,
            SetFile::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEventsImpl, const OFFSET: isize>() -> IFeedEventsVtbl {
        unsafe extern "system" fn Error<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDeleted(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedRenamed(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedUrlChanged(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMoved<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMoved(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloading(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloadCompleted(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedItemCountChanged(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), itemcounttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedEvents>,
            ::windows::core::GetTrustLevel,
            Error::<Impl, OFFSET>,
            FeedDeleted::<Impl, OFFSET>,
            FeedRenamed::<Impl, OFFSET>,
            FeedUrlChanged::<Impl, OFFSET>,
            FeedMoved::<Impl, OFFSET>,
            FeedDownloading::<Impl, OFFSET>,
            FeedDownloadCompleted::<Impl, OFFSET>,
            FeedItemCountChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedFolder {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedFolder";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderImpl, const OFFSET: isize>() -> IFeedFolderVtbl {
        unsafe extern "system" fn Feeds<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Feeds(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subfolders(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFeed(&*(&feedname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&feedurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubfolder(&*(&foldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(&*(&feedname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(&*(&feedname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsSubfolder(&*(&foldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubfolder(&*(&foldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&foldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(&*(&newparentpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isroot: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoot(::core::mem::transmute_copy(&isroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalUnreadItemCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalItemCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(scope, mask, ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedFolder>,
            ::windows::core::GetTrustLevel,
            Feeds::<Impl, OFFSET>,
            Subfolders::<Impl, OFFSET>,
            CreateFeed::<Impl, OFFSET>,
            CreateSubfolder::<Impl, OFFSET>,
            ExistsFeed::<Impl, OFFSET>,
            GetFeed::<Impl, OFFSET>,
            ExistsSubfolder::<Impl, OFFSET>,
            GetSubfolder::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Rename::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            IsRoot::<Impl, OFFSET>,
            TotalUnreadItemCount::<Impl, OFFSET>,
            TotalItemCount::<Impl, OFFSET>,
            GetWatcher::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedFolderEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedFolderEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolderEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEventsImpl, const OFFSET: isize>() -> IFeedFolderEventsVtbl {
        unsafe extern "system" fn Error<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderAdded<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderAdded(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderDeleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderDeleted(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderRenamed<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderRenamed(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderMovedFrom(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderMovedTo(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderItemCountChanged(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), itemcounttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedAdded<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedAdded(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDeleted(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedRenamed(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedUrlChanged(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMovedFrom(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMovedTo(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&oldpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloading(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloadCompleted(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), error) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedItemCountChanged(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), itemcounttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedFolderEvents>,
            ::windows::core::GetTrustLevel,
            Error::<Impl, OFFSET>,
            FolderAdded::<Impl, OFFSET>,
            FolderDeleted::<Impl, OFFSET>,
            FolderRenamed::<Impl, OFFSET>,
            FolderMovedFrom::<Impl, OFFSET>,
            FolderMovedTo::<Impl, OFFSET>,
            FolderItemCountChanged::<Impl, OFFSET>,
            FeedAdded::<Impl, OFFSET>,
            FeedDeleted::<Impl, OFFSET>,
            FeedRenamed::<Impl, OFFSET>,
            FeedUrlChanged::<Impl, OFFSET>,
            FeedMovedFrom::<Impl, OFFSET>,
            FeedMovedTo::<Impl, OFFSET>,
            FeedDownloading::<Impl, OFFSET>,
            FeedDownloadCompleted::<Impl, OFFSET>,
            FeedItemCountChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedItem {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItemImpl, const OFFSET: isize>() -> IFeedItemVtbl {
        unsafe extern "system" fn Xml<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(includeflags, ::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&title)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linkurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&linkurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid(::core::mem::transmute_copy(&itemguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate(::core::mem::transmute_copy(&pubdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comments(::core::mem::transmute_copy(&comments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author(::core::mem::transmute_copy(&author)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enclosure(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead(::core::mem::transmute_copy(&isread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsRead(isread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId(::core::mem::transmute_copy(&itemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&itemurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime(::core::mem::transmute_copy(&lastdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: IFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&modified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedItem>,
            ::windows::core::GetTrustLevel,
            Xml::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Link::<Impl, OFFSET>,
            Guid::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            PubDate::<Impl, OFFSET>,
            Comments::<Impl, OFFSET>,
            Author::<Impl, OFFSET>,
            Enclosure::<Impl, OFFSET>,
            IsRead::<Impl, OFFSET>,
            SetIsRead::<Impl, OFFSET>,
            LocalId::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            LastDownloadTime::<Impl, OFFSET>,
            Modified::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedItem2Impl: Sized + IFeedItemImpl + IDispatchImpl {
    fn EffectiveId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedItem2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedItem2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem2Impl, const OFFSET: isize>() -> IFeedItem2Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IFeedItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveId(::core::mem::transmute_copy(&effectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFeedItem2>, ::windows::core::GetTrustLevel, EffectiveId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedsEnumImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedsEnum {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedsEnum";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedsEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnumImpl, const OFFSET: isize>() -> IFeedsEnumVtbl {
        unsafe extern "system" fn Count<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&enumvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFeedsEnum>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFeedsManager {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IFeedsManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IFeedsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManagerImpl, const OFFSET: isize>() -> IFeedsManagerVtbl {
        unsafe extern "system" fn RootFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootFolder(::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subscribed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscribed(&*(&feedurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&subscribed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(&*(&feedpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(&*(&feedpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeedByUrl(&*(&feedurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFolder(&*(&folderpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&exists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolder(&*(&folderpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&disp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFeed(&*(&feedpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFolder(&*(&folderpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSync<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSync(action) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInterval(::core::mem::transmute_copy(&minutes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultInterval(minutes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncSyncAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normalize<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxmlin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedxmlout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normalize(&*(&feedxmlin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&feedxmlout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCountLimit(::core::mem::transmute_copy(&itemcountlimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFeedsManager>,
            ::windows::core::GetTrustLevel,
            RootFolder::<Impl, OFFSET>,
            IsSubscribed::<Impl, OFFSET>,
            ExistsFeed::<Impl, OFFSET>,
            GetFeed::<Impl, OFFSET>,
            GetFeedByUrl::<Impl, OFFSET>,
            ExistsFolder::<Impl, OFFSET>,
            GetFolder::<Impl, OFFSET>,
            DeleteFeed::<Impl, OFFSET>,
            DeleteFolder::<Impl, OFFSET>,
            BackgroundSync::<Impl, OFFSET>,
            BackgroundSyncStatus::<Impl, OFFSET>,
            DefaultInterval::<Impl, OFFSET>,
            SetDefaultInterval::<Impl, OFFSET>,
            AsyncSyncAll::<Impl, OFFSET>,
            Normalize::<Impl, OFFSET>,
            ItemCountLimit::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPAudioRenderConfigImpl: Sized {
    fn audioOutputDevice();
    fn SetaudioOutputDevice();
}
impl ::windows::core::RuntimeName for IWMPAudioRenderConfig {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPAudioRenderConfig";
}
impl IWMPAudioRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfigImpl, const OFFSET: isize>() -> IWMPAudioRenderConfigVtbl {
        unsafe extern "system" fn audioOutputDevice<Impl: IWMPAudioRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).audioOutputDevice(&*(&pbstroutputdevice as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetaudioOutputDevice<Impl: IWMPAudioRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetaudioOutputDevice(&*(&bstroutputdevice as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPAudioRenderConfig>, ::windows::core::GetTrustLevel, audioOutputDevice::<Impl, OFFSET>, SetaudioOutputDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromImpl: Sized + IDispatchImpl {
    fn driveSpecifier();
    fn playlist();
    fn eject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPCdrom {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCdrom";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromImpl, const OFFSET: isize>() -> IWMPCdromVtbl {
        unsafe extern "system" fn driveSpecifier<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).driveSpecifier(&*(&pbstrdrive as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlist<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playlist(::core::mem::transmute_copy(&ppplaylist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eject<Impl: IWMPCdromImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).eject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPCdrom>, ::windows::core::GetTrustLevel, driveSpecifier::<Impl, OFFSET>, playlist::<Impl, OFFSET>, eject::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPCdromBurn {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCdromBurn";
}
impl IWMPCdromBurnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurnImpl, const OFFSET: isize>() -> IWMPCdromBurnVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAvailable(&*(&bstritem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pisavailable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstritem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn label<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).label(&*(&pbstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setlabel<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setlabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn burnFormat<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).burnFormat(pwmpbf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnFormat<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetburnFormat(wmpbf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn burnPlaylist<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).burnPlaylist(::core::mem::transmute_copy(&ppplaylist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnPlaylist<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetburnPlaylist(&*(&pplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn refreshStatus<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).refreshStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn burnState<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).burnState(pwmpbs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn burnProgress<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).burnProgress(plprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startBurn<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startBurn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopBurn<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stopBurn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn erase<Impl: IWMPCdromBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).erase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPCdromBurn>,
            ::windows::core::GetTrustLevel,
            isAvailable::<Impl, OFFSET>,
            getItemInfo::<Impl, OFFSET>,
            label::<Impl, OFFSET>,
            Setlabel::<Impl, OFFSET>,
            burnFormat::<Impl, OFFSET>,
            SetburnFormat::<Impl, OFFSET>,
            burnPlaylist::<Impl, OFFSET>,
            SetburnPlaylist::<Impl, OFFSET>,
            refreshStatus::<Impl, OFFSET>,
            burnState::<Impl, OFFSET>,
            burnProgress::<Impl, OFFSET>,
            startBurn::<Impl, OFFSET>,
            stopBurn::<Impl, OFFSET>,
            erase::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
    fn getByDriveSpecifier();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPCdromCollection {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCdromCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollectionImpl, const OFFSET: isize>() -> IWMPCdromCollectionVtbl {
        unsafe extern "system" fn count<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(lindex, ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Impl: IWMPCdromCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcdrom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByDriveSpecifier(&*(&bstrdrivespecifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcdrom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPCdromCollection>, ::windows::core::GetTrustLevel, count::<Impl, OFFSET>, item::<Impl, OFFSET>, getByDriveSpecifier::<Impl, OFFSET>)
    }
}
pub trait IWMPCdromRipImpl: Sized {
    fn ripState();
    fn ripProgress();
    fn startRip();
    fn stopRip();
}
impl ::windows::core::RuntimeName for IWMPCdromRip {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCdromRip";
}
impl IWMPCdromRipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRipImpl, const OFFSET: isize>() -> IWMPCdromRipVtbl {
        unsafe extern "system" fn ripState<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ripState(pwmprs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ripProgress<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ripProgress(plprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startRip<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startRip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopRip<Impl: IWMPCdromRipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stopRip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPCdromRip>, ::windows::core::GetTrustLevel, ripState::<Impl, OFFSET>, ripProgress::<Impl, OFFSET>, startRip::<Impl, OFFSET>, stopRip::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPClosedCaption {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPClosedCaption";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaptionImpl, const OFFSET: isize>() -> IWMPClosedCaptionVtbl {
        unsafe extern "system" fn SAMIStyle<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAMIStyle(&*(&pbstrsamistyle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSAMIStyle<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSAMIStyle(&*(&bstrsamistyle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SAMILang<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAMILang(&*(&pbstrsamilang as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSAMILang<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSAMILang(&*(&bstrsamilang as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SAMIFileName<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAMIFileName(&*(&pbstrsamifilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSAMIFileName<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSAMIFileName(&*(&bstrsamifilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn captioningId<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).captioningId(&*(&pbstrcaptioningid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcaptioningId<Impl: IWMPClosedCaptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcaptioningId(&*(&bstrcaptioningid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPClosedCaption>,
            ::windows::core::GetTrustLevel,
            SAMIStyle::<Impl, OFFSET>,
            SetSAMIStyle::<Impl, OFFSET>,
            SAMILang::<Impl, OFFSET>,
            SetSAMILang::<Impl, OFFSET>,
            SAMIFileName::<Impl, OFFSET>,
            SetSAMIFileName::<Impl, OFFSET>,
            captioningId::<Impl, OFFSET>,
            SetcaptioningId::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPClosedCaption2Impl: Sized + IWMPClosedCaptionImpl + IDispatchImpl {
    fn SAMILangCount();
    fn getSAMILangName();
    fn getSAMILangID();
    fn SAMIStyleCount();
    fn getSAMIStyleName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPClosedCaption2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPClosedCaption2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2Impl, const OFFSET: isize>() -> IWMPClosedCaption2Vtbl {
        unsafe extern "system" fn SAMILangCount<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAMILangCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSAMILangName<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSAMILangName(nindex, &*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSAMILangID<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSAMILangID(nindex, pllangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SAMIStyleCount<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SAMIStyleCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSAMIStyleName<Impl: IWMPClosedCaption2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getSAMIStyleName(nindex, &*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPClosedCaption2>, ::windows::core::GetTrustLevel, SAMILangCount::<Impl, OFFSET>, getSAMILangName::<Impl, OFFSET>, getSAMILangID::<Impl, OFFSET>, SAMIStyleCount::<Impl, OFFSET>, getSAMIStyleName::<Impl, OFFSET>)
    }
}
pub trait IWMPContentContainerImpl: Sized {
    fn GetID();
    fn GetPrice();
    fn GetType();
    fn GetContentCount();
    fn GetContentPrice();
    fn GetContentID();
}
impl ::windows::core::RuntimeName for IWMPContentContainer {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPContentContainer";
}
impl IWMPContentContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerImpl, const OFFSET: isize>() -> IWMPContentContainerVtbl {
        unsafe extern "system" fn GetID<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID(::core::mem::transmute_copy(&pcontentid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrice<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrice(::core::mem::transmute_copy(&pbstrprice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pbstrtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentCount<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentCount(::core::mem::transmute_copy(&pccontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPrice<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentPrice(idxcontent, ::core::mem::transmute_copy(&pbstrprice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentID<Impl: IWMPContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentID(idxcontent, ::core::mem::transmute_copy(&pcontentid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPContentContainer>, ::windows::core::GetTrustLevel, GetID::<Impl, OFFSET>, GetPrice::<Impl, OFFSET>, GetType::<Impl, OFFSET>, GetContentCount::<Impl, OFFSET>, GetContentPrice::<Impl, OFFSET>, GetContentID::<Impl, OFFSET>)
    }
}
pub trait IWMPContentContainerListImpl: Sized {
    fn GetTransactionType();
    fn GetContainerCount();
    fn GetContainer();
}
impl ::windows::core::RuntimeName for IWMPContentContainerList {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPContentContainerList";
}
impl IWMPContentContainerListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerListImpl, const OFFSET: isize>() -> IWMPContentContainerListVtbl {
        unsafe extern "system" fn GetTransactionType<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionType(::core::mem::transmute_copy(&pwmptt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerCount<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerCount(::core::mem::transmute_copy(&pccontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: IWMPContentContainerListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainer(idxcontainer, ::core::mem::transmute_copy(&ppcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPContentContainerList>, ::windows::core::GetTrustLevel, GetTransactionType::<Impl, OFFSET>, GetContainerCount::<Impl, OFFSET>, GetContainer::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPContentPartner {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPContentPartner";
}
impl IWMPContentPartnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerImpl, const OFFSET: isize>() -> IWMPContentPartnerVtbl {
        unsafe extern "system" fn SetCallback<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCallback(&*(&pcallback as *const <IWMPContentPartnerCallback as ::windows::core::Abi>::Abi as *const <IWMPContentPartnerCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(r#type, &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemInfo<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemInfo(&*(&bstrinfoname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPartnerInfo<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentPartnerInfo(&*(&bstrinfoname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommands(
                &*(&location as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&plocationcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&itemlocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                citemids,
                prgitemids,
                ::core::mem::transmute_copy(&pcitemids),
                ::core::mem::transmute_copy(&pprgitems),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeCommand<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeCommand(
                dwcommandid,
                &*(&location as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&plocationcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&itemlocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                citemids,
                rgitemids,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBuySilent<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBuySilent(&*(&pinfo as *const <IWMPContentContainerList as ::windows::core::Abi>::Abi as *const <IWMPContentContainerList as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrtotalprice), ::core::mem::transmute_copy(&psilentok)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buy<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buy(&*(&pinfo as *const <IWMPContentContainerList as ::windows::core::Abi>::Abi as *const <IWMPContentContainerList as ::windows::core::DefaultType>::DefaultType), cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamingURL<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamingURL(st, &*(&pstreamcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download(&*(&pinfo as *const <IWMPContentContainerList as ::windows::core::Abi>::Abi as *const <IWMPContentContainerList as ::windows::core::DefaultType>::DefaultType), cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadTrackComplete<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadTrackComplete(hrresult, contentid, &*(&downloadtrackparam as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshLicense<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: i16, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshLicense(
                dwcookie,
                flocal,
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                r#type,
                contentid,
                &*(&bstrrefreshreason as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&preasoncontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCatalogURL<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCatalogURL(dwcatalogversion, dwcatalogschemaversion, cataloglcid, ::core::mem::transmute_copy(&pdwnewcatalogversion), ::core::mem::transmute_copy(&pbstrcatalogurl), ::core::mem::transmute_copy(&pexpirationdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplate<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, clicklocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrviewparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplate(
                task,
                &*(&location as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&clicklocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pclickcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrviewparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pbstrtemplateurl),
                ::core::mem::transmute_copy(&ptemplatesize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDevice<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDevice(&*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListContents<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListContents(
                &*(&location as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrlisttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwlistcookie,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Login<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Login(&*(&userinfo as *const <super::super::System::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::super::System::Com::BLOB as ::windows::core::DefaultType>::DefaultType), &*(&pwdinfo as *const <super::super::System::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::super::System::Com::BLOB as ::windows::core::DefaultType>::DefaultType), fusedcachedcreds, foktocache) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(&*(&userinfo as *const <super::super::System::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::super::System::Com::BLOB as ::windows::core::DefaultType>::DefaultType), &*(&pwdinfo as *const <super::super::System::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::super::System::Com::BLOB as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logout<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessage<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessage(&*(&bstrmsg as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrparam as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StationEvent<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StationEvent(&*(&bstrstationeventtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), stationid, playlistindex, trackid, &*(&trackdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwsecondsplayed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareContainerListPrices<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistbase: ::windows::core::RawPtr, plistcompare: ::windows::core::RawPtr, presult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareContainerListPrices(&*(&plistbase as *const <IWMPContentContainerList as ::windows::core::Abi>::Abi as *const <IWMPContentContainerList as ::windows::core::DefaultType>::DefaultType), &*(&plistcompare as *const <IWMPContentContainerList as ::windows::core::Abi>::Abi as *const <IWMPContentContainerList as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermission<Impl: IWMPContentPartnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyPermission(&*(&bstrpermission as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPContentPartner>,
            ::windows::core::GetTrustLevel,
            SetCallback::<Impl, OFFSET>,
            Notify::<Impl, OFFSET>,
            GetItemInfo::<Impl, OFFSET>,
            GetContentPartnerInfo::<Impl, OFFSET>,
            GetCommands::<Impl, OFFSET>,
            InvokeCommand::<Impl, OFFSET>,
            CanBuySilent::<Impl, OFFSET>,
            Buy::<Impl, OFFSET>,
            GetStreamingURL::<Impl, OFFSET>,
            Download::<Impl, OFFSET>,
            DownloadTrackComplete::<Impl, OFFSET>,
            RefreshLicense::<Impl, OFFSET>,
            GetCatalogURL::<Impl, OFFSET>,
            GetTemplate::<Impl, OFFSET>,
            UpdateDevice::<Impl, OFFSET>,
            GetListContents::<Impl, OFFSET>,
            Login::<Impl, OFFSET>,
            Authenticate::<Impl, OFFSET>,
            Logout::<Impl, OFFSET>,
            SendMessage::<Impl, OFFSET>,
            StationEvent::<Impl, OFFSET>,
            CompareContainerListPrices::<Impl, OFFSET>,
            VerifyPermission::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IWMPContentPartnerCallback {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPContentPartnerCallback";
}
impl IWMPContentPartnerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>() -> IWMPContentPartnerCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(r#type, &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuyComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuyComplete(hrresult, dwbuycookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadTrack<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadTrack(cookie, &*(&bstrtrackurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwservicetrackid, &*(&bstrdownloadparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), hrdownload) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCatalogVersion<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCatalogVersion(::core::mem::transmute_copy(&pdwversion), ::core::mem::transmute_copy(&pdwschemaversion), ::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDeviceComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDeviceComplete(&*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeView<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeView(
                &*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddListContents<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddListContents(dwlistcookie, citems, prgitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListContentsComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListContentsComplete(dwlistcookie, hrsuccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessageComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageComplete(
                &*(&bstrmsg as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrparam as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresult as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentIDsInLibrary(::core::mem::transmute_copy(&pccontentids), ::core::mem::transmute_copy(&pprgids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshLicenseComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshLicenseComplete(dwcookie, contentid, hrrefresh) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPopup<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowPopup(lindex, &*(&bstrparameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermissionComplete<Impl: IWMPContentPartnerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyPermissionComplete(&*(&bstrpermission as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), hrpermission) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPContentPartnerCallback>,
            ::windows::core::GetTrustLevel,
            Notify::<Impl, OFFSET>,
            BuyComplete::<Impl, OFFSET>,
            DownloadTrack::<Impl, OFFSET>,
            GetCatalogVersion::<Impl, OFFSET>,
            UpdateDeviceComplete::<Impl, OFFSET>,
            ChangeView::<Impl, OFFSET>,
            AddListContents::<Impl, OFFSET>,
            ListContentsComplete::<Impl, OFFSET>,
            SendMessageComplete::<Impl, OFFSET>,
            GetContentIDsInLibrary::<Impl, OFFSET>,
            RefreshLicenseComplete::<Impl, OFFSET>,
            ShowPopup::<Impl, OFFSET>,
            VerifyPermissionComplete::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPControls {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPControls";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControlsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControlsImpl, const OFFSET: isize>() -> IWMPControlsVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAvailable(&*(&bstritem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pisavailable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn play<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).play() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stop<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pause<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fastForward<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fastForward() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fastReverse<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fastReverse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPosition<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentPosition(pdcurrentposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPosition<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentPosition(dcurrentposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPositionString<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentPositionString(&*(&pbstrcurrentposition as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).next() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previous<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).previous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentItem(::core::mem::transmute_copy(&ppiwmpmedia)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentItem(&*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMarker<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentMarker(plmarker) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMarker<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentMarker(lmarker) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playItem<Impl: IWMPControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playItem(&*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPControls>,
            ::windows::core::GetTrustLevel,
            isAvailable::<Impl, OFFSET>,
            play::<Impl, OFFSET>,
            stop::<Impl, OFFSET>,
            pause::<Impl, OFFSET>,
            fastForward::<Impl, OFFSET>,
            fastReverse::<Impl, OFFSET>,
            currentPosition::<Impl, OFFSET>,
            SetcurrentPosition::<Impl, OFFSET>,
            currentPositionString::<Impl, OFFSET>,
            next::<Impl, OFFSET>,
            previous::<Impl, OFFSET>,
            currentItem::<Impl, OFFSET>,
            SetcurrentItem::<Impl, OFFSET>,
            currentMarker::<Impl, OFFSET>,
            SetcurrentMarker::<Impl, OFFSET>,
            playItem::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls2Impl: Sized + IWMPControlsImpl + IDispatchImpl {
    fn step();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPControls2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPControls2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls2Impl, const OFFSET: isize>() -> IWMPControls2Vtbl {
        unsafe extern "system" fn step<Impl: IWMPControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).step(lstep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPControls2>, ::windows::core::GetTrustLevel, step::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls3Impl: Sized + IWMPControls2Impl + IWMPControlsImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPControls3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPControls3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3Impl, const OFFSET: isize>() -> IWMPControls3Vtbl {
        unsafe extern "system" fn audioLanguageCount<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).audioLanguageCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAudioLanguageID<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAudioLanguageID(lindex, pllangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAudioLanguageDescription<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAudioLanguageDescription(lindex, &*(&pbstrlangdesc as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentAudioLanguage<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentAudioLanguage(pllangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentAudioLanguage(llangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentAudioLanguageIndex(plindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentAudioLanguageIndex(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLanguageName<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLanguageName(llangid, &*(&pbstrlangname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPositionTimecode<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentPositionTimecode(&*(&bstrtimecode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Impl: IWMPControls3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentPositionTimecode(&*(&bstrtimecode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPControls3>,
            ::windows::core::GetTrustLevel,
            audioLanguageCount::<Impl, OFFSET>,
            getAudioLanguageID::<Impl, OFFSET>,
            getAudioLanguageDescription::<Impl, OFFSET>,
            currentAudioLanguage::<Impl, OFFSET>,
            SetcurrentAudioLanguage::<Impl, OFFSET>,
            currentAudioLanguageIndex::<Impl, OFFSET>,
            SetcurrentAudioLanguageIndex::<Impl, OFFSET>,
            getLanguageName::<Impl, OFFSET>,
            currentPositionTimecode::<Impl, OFFSET>,
            SetcurrentPositionTimecode::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPConvertImpl: Sized {
    fn ConvertFile();
    fn GetErrorURL();
}
impl ::windows::core::RuntimeName for IWMPConvert {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPConvert";
}
impl IWMPConvertVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvertImpl, const OFFSET: isize>() -> IWMPConvertVtbl {
        unsafe extern "system" fn ConvertFile<Impl: IWMPConvertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFile(
                &*(&bstrinputfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestinationfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstroutputfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorURL<Impl: IWMPConvertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorURL(&*(&pbstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPConvert>, ::windows::core::GetTrustLevel, ConvertFile::<Impl, OFFSET>, GetErrorURL::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPCore {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCore";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCoreImpl, const OFFSET: isize>() -> IWMPCoreVtbl {
        unsafe extern "system" fn close<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).URL(&*(&pbstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetURL(&*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openState<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).openState(pwmpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playState<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playState(pwmpps) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn controls<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).controls(::core::mem::transmute_copy(&ppcontrol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn settings<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).settings(::core::mem::transmute_copy(&ppsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMedia<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentMedia(::core::mem::transmute_copy(&ppmedia)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMedia<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentMedia(&*(&pmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mediaCollection(::core::mem::transmute_copy(&ppmediacollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlistCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playlistCollection(::core::mem::transmute_copy(&ppplaylistcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn versionInfo<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).versionInfo(&*(&pbstrversioninfo as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn launchURL<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).launchURL(&*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn network<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqni: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).network(::core::mem::transmute_copy(&ppqni)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPlaylist<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentPlaylist(::core::mem::transmute_copy(&pppl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPlaylist<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcurrentPlaylist(&*(&ppl as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cdromCollection<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).cdromCollection(::core::mem::transmute_copy(&ppcdromcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn closedCaption<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).closedCaption(::core::mem::transmute_copy(&ppclosedcaption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOnline<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfonline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isOnline(pfonline) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn error<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).error(::core::mem::transmute_copy(&pperror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Impl: IWMPCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).status(&*(&pbstrstatus as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPCore>,
            ::windows::core::GetTrustLevel,
            close::<Impl, OFFSET>,
            URL::<Impl, OFFSET>,
            SetURL::<Impl, OFFSET>,
            openState::<Impl, OFFSET>,
            playState::<Impl, OFFSET>,
            controls::<Impl, OFFSET>,
            settings::<Impl, OFFSET>,
            currentMedia::<Impl, OFFSET>,
            SetcurrentMedia::<Impl, OFFSET>,
            mediaCollection::<Impl, OFFSET>,
            playlistCollection::<Impl, OFFSET>,
            versionInfo::<Impl, OFFSET>,
            launchURL::<Impl, OFFSET>,
            network::<Impl, OFFSET>,
            currentPlaylist::<Impl, OFFSET>,
            SetcurrentPlaylist::<Impl, OFFSET>,
            cdromCollection::<Impl, OFFSET>,
            closedCaption::<Impl, OFFSET>,
            isOnline::<Impl, OFFSET>,
            error::<Impl, OFFSET>,
            status::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore2Impl: Sized + IWMPCoreImpl + IDispatchImpl {
    fn dvd();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPCore2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCore2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore2Impl, const OFFSET: isize>() -> IWMPCore2Vtbl {
        unsafe extern "system" fn dvd<Impl: IWMPCore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dvd(::core::mem::transmute_copy(&ppdvd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPCore2>, ::windows::core::GetTrustLevel, dvd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore3Impl: Sized + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
    fn newPlaylist();
    fn newMedia();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPCore3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPCore3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3Impl, const OFFSET: isize>() -> IWMPCore3Vtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPCore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newPlaylist(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppplaylist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn newMedia<Impl: IWMPCore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newMedia(&*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmedia)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPCore3>, ::windows::core::GetTrustLevel, newPlaylist::<Impl, OFFSET>, newMedia::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDVDImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn domain();
    fn topMenu();
    fn titleMenu();
    fn back();
    fn resume();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPDVD {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPDVD";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDVDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVDImpl, const OFFSET: isize>() -> IWMPDVDVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAvailable(&*(&bstritem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pisavailable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn domain<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).domain(&*(&strdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn topMenu<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).topMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn titleMenu<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).titleMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn back<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).back() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn resume<Impl: IWMPDVDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPDVD>, ::windows::core::GetTrustLevel, isAvailable::<Impl, OFFSET>, domain::<Impl, OFFSET>, topMenu::<Impl, OFFSET>, titleMenu::<Impl, OFFSET>, back::<Impl, OFFSET>, resume::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadCollectionImpl: Sized + IDispatchImpl {
    fn id();
    fn count();
    fn item();
    fn startDownload();
    fn removeItem();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPDownloadCollection {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPDownloadCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>() -> IWMPDownloadCollectionVtbl {
        unsafe extern "system" fn id<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).id(plid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn count<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(litem, ::core::mem::transmute_copy(&ppdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDownload<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startDownload(&*(&bstrsourceurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeItem(litem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWMPDownloadCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPDownloadCollection>, ::windows::core::GetTrustLevel, id::<Impl, OFFSET>, count::<Impl, OFFSET>, item::<Impl, OFFSET>, startDownload::<Impl, OFFSET>, removeItem::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPDownloadItem {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPDownloadItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItemImpl, const OFFSET: isize>() -> IWMPDownloadItemVtbl {
        unsafe extern "system" fn sourceURL<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).sourceURL(&*(&pbstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn size<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).size(plsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type(&*(&pbstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn progress<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).progress(plprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn downloadState<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).downloadState(pwmpsdls) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pause<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn resume<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cancel<Impl: IWMPDownloadItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPDownloadItem>, ::windows::core::GetTrustLevel, sourceURL::<Impl, OFFSET>, size::<Impl, OFFSET>, r#type::<Impl, OFFSET>, progress::<Impl, OFFSET>, downloadState::<Impl, OFFSET>, pause::<Impl, OFFSET>, resume::<Impl, OFFSET>, cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadItem2Impl: Sized + IWMPDownloadItemImpl + IDispatchImpl {
    fn getItemInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPDownloadItem2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPDownloadItem2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem2Impl, const OFFSET: isize>() -> IWMPDownloadItem2Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPDownloadItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPDownloadItem2>, ::windows::core::GetTrustLevel, getItemInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadManagerImpl: Sized + IDispatchImpl {
    fn getDownloadCollection();
    fn createDownloadCollection();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPDownloadManager {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPDownloadManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManagerImpl, const OFFSET: isize>() -> IWMPDownloadManagerVtbl {
        unsafe extern "system" fn getDownloadCollection<Impl: IWMPDownloadManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDownloadCollection(lcollectionid, ::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDownloadCollection<Impl: IWMPDownloadManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createDownloadCollection(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPDownloadManager>, ::windows::core::GetTrustLevel, getDownloadCollection::<Impl, OFFSET>, createDownloadCollection::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPEffects {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEffects";
}
impl IWMPEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffectsImpl, const OFFSET: isize>() -> IWMPEffectsVtbl {
        unsafe extern "system" fn Render<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Render(&*(&plevels as *const <TimedLevel as ::windows::core::Abi>::Abi as *const <TimedLevel as ::windows::core::DefaultType>::DefaultType), &*(&hdc as *const <super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), &*(&prc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaInfo<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaInfo(lchannelcount, lsamplerate, &*(&bstrtitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(pdwcapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle(&*(&bstrtitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresetTitle<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresetTitle(npreset, &*(&bstrpresettitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresetCount<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresetCount(pnpresetcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPreset<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentPreset(npreset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPreset<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPreset(pnpreset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPropertyPage(&*(&hwndowner as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoFullscreen<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoFullscreen(&*(&ffullscreen as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderFullScreen<Impl: IWMPEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderFullScreen(&*(&plevels as *const <TimedLevel as ::windows::core::Abi>::Abi as *const <TimedLevel as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPEffects>,
            ::windows::core::GetTrustLevel,
            Render::<Impl, OFFSET>,
            MediaInfo::<Impl, OFFSET>,
            GetCapabilities::<Impl, OFFSET>,
            GetTitle::<Impl, OFFSET>,
            GetPresetTitle::<Impl, OFFSET>,
            GetPresetCount::<Impl, OFFSET>,
            SetCurrentPreset::<Impl, OFFSET>,
            GetCurrentPreset::<Impl, OFFSET>,
            DisplayPropertyPage::<Impl, OFFSET>,
            GoFullscreen::<Impl, OFFSET>,
            RenderFullScreen::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPEffects2Impl: Sized + IWMPEffectsImpl {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn NotifyNewMedia();
    fn OnWindowMessage();
    fn RenderWindowed();
}
impl ::windows::core::RuntimeName for IWMPEffects2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEffects2";
}
impl IWMPEffects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2Impl, const OFFSET: isize>() -> IWMPEffects2Vtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCore(&*(&pplayer as *const <IWMPCore as ::windows::core::Abi>::Abi as *const <IWMPCore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyNewMedia<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyNewMedia(&*(&pmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowMessage(
                msg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&plresultparam as *const <super::super::Foundation::LRESULT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LRESULT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderWindowed<Impl: IWMPEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderWindowed(&*(&pdata as *const <TimedLevel as ::windows::core::Abi>::Abi as *const <TimedLevel as ::windows::core::DefaultType>::DefaultType), &*(&frequiredrender as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPEffects2>, ::windows::core::GetTrustLevel, SetCore::<Impl, OFFSET>, Create::<Impl, OFFSET>, Destroy::<Impl, OFFSET>, NotifyNewMedia::<Impl, OFFSET>, OnWindowMessage::<Impl, OFFSET>, RenderWindowed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorImpl: Sized + IDispatchImpl {
    fn clearErrorQueue();
    fn errorCount();
    fn item();
    fn webHelp();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPError {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPError";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorImpl, const OFFSET: isize>() -> IWMPErrorVtbl {
        unsafe extern "system" fn clearErrorQueue<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).clearErrorQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorCount<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorCount(plnumerrors) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(dwindex, ::core::mem::transmute_copy(&pperroritem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn webHelp<Impl: IWMPErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).webHelp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPError>, ::windows::core::GetTrustLevel, clearErrorQueue::<Impl, OFFSET>, errorCount::<Impl, OFFSET>, item::<Impl, OFFSET>, webHelp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItemImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn errorDescription();
    fn errorContext();
    fn remedy();
    fn customUrl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPErrorItem {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPErrorItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItemImpl, const OFFSET: isize>() -> IWMPErrorItemVtbl {
        unsafe extern "system" fn errorCode<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorCode(phr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorDescription<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorDescription(&*(&pbstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorContext<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).errorContext(&*(&pvarcontext as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remedy<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).remedy(plremedy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customUrl<Impl: IWMPErrorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).customUrl(&*(&pbstrcustomurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPErrorItem>, ::windows::core::GetTrustLevel, errorCode::<Impl, OFFSET>, errorDescription::<Impl, OFFSET>, errorContext::<Impl, OFFSET>, remedy::<Impl, OFFSET>, customUrl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItem2Impl: Sized + IWMPErrorItemImpl + IDispatchImpl {
    fn condition();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPErrorItem2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPErrorItem2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem2Impl, const OFFSET: isize>() -> IWMPErrorItem2Vtbl {
        unsafe extern "system" fn condition<Impl: IWMPErrorItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).condition(plcondition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPErrorItem2>, ::windows::core::GetTrustLevel, condition::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEvents";
}
impl IWMPEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEventsImpl, const OFFSET: isize>() -> IWMPEventsVtbl {
        unsafe extern "system" fn OpenStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenStateChange(newstate).into()
        }
        unsafe extern "system" fn PlayStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayStateChange(newstate).into()
        }
        unsafe extern "system" fn AudioLanguageChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AudioLanguageChange(langid).into()
        }
        unsafe extern "system" fn StatusChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StatusChange().into()
        }
        unsafe extern "system" fn ScriptCommand<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sctype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, param: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScriptCommand(&*(&sctype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&param as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NewStream<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewStream().into()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(result).into()
        }
        unsafe extern "system" fn Buffering<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Buffering(start).into()
        }
        unsafe extern "system" fn Error<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn Warning<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Warning(warningtype, param, &*(&description as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndOfStream<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOfStream(result).into()
        }
        unsafe extern "system" fn PositionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PositionChange(oldposition, newposition).into()
        }
        unsafe extern "system" fn MarkerHit<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarkerHit(markernum).into()
        }
        unsafe extern "system" fn DurationUnitChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DurationUnitChange(newdurationunit).into()
        }
        unsafe extern "system" fn CdromMediaChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromMediaChange(cdromnum).into()
        }
        unsafe extern "system" fn PlaylistChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlist: ::windows::core::RawPtr, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistChange(&*(&playlist as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), change).into()
        }
        unsafe extern "system" fn CurrentPlaylistChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentPlaylistChange(change).into()
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentPlaylistItemAvailable(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaChange(&*(&item as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentMediaItemAvailable(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentItemChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentItemChange(&*(&pdispmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCollectionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionChange().into()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionAttributeStringAdded(&*(&bstrattribname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrattribval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionAttributeStringRemoved(&*(&bstrattribname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrattribval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroldattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .MediaCollectionAttributeStringChanged(
                    &*(&bstrattribname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                    &*(&bstroldattribval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                    &*(&bstrnewattribval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn PlaylistCollectionChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionChange().into()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistAdded(&*(&bstrplaylistname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistRemoved(&*(&bstrplaylistname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfisdeleted: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistSetAsDeleted(&*(&bstrplaylistname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), varfisdeleted).into()
        }
        unsafe extern "system" fn ModeChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newvalue: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModeChange(&*(&modename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), newvalue).into()
        }
        unsafe extern "system" fn MediaError<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediaobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaError(&*(&pmediaobject as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenPlaylistSwitch(&*(&pitem as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DomainChange(&*(&strdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchedToPlayerApplication().into()
        }
        unsafe extern "system" fn SwitchedToControl<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchedToControl().into()
        }
        unsafe extern "system" fn PlayerDockedStateChange<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayerDockedStateChange().into()
        }
        unsafe extern "system" fn PlayerReconnect<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayerReconnect().into()
        }
        unsafe extern "system" fn Click<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Click(nbutton, nshiftstate, fx, fy).into()
        }
        unsafe extern "system" fn DoubleClick<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoubleClick(nbutton, nshiftstate, fx, fy).into()
        }
        unsafe extern "system" fn KeyDown<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyDown(nkeycode, nshiftstate).into()
        }
        unsafe extern "system" fn KeyPress<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyPress(nkeyascii).into()
        }
        unsafe extern "system" fn KeyUp<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyUp(nkeycode, nshiftstate).into()
        }
        unsafe extern "system" fn MouseDown<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseDown(nbutton, nshiftstate, fx, fy).into()
        }
        unsafe extern "system" fn MouseMove<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseMove(nbutton, nshiftstate, fx, fy).into()
        }
        unsafe extern "system" fn MouseUp<Impl: IWMPEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseUp(nbutton, nshiftstate, fx, fy).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPEvents>,
            ::windows::core::GetTrustLevel,
            OpenStateChange::<Impl, OFFSET>,
            PlayStateChange::<Impl, OFFSET>,
            AudioLanguageChange::<Impl, OFFSET>,
            StatusChange::<Impl, OFFSET>,
            ScriptCommand::<Impl, OFFSET>,
            NewStream::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            Buffering::<Impl, OFFSET>,
            Error::<Impl, OFFSET>,
            Warning::<Impl, OFFSET>,
            EndOfStream::<Impl, OFFSET>,
            PositionChange::<Impl, OFFSET>,
            MarkerHit::<Impl, OFFSET>,
            DurationUnitChange::<Impl, OFFSET>,
            CdromMediaChange::<Impl, OFFSET>,
            PlaylistChange::<Impl, OFFSET>,
            CurrentPlaylistChange::<Impl, OFFSET>,
            CurrentPlaylistItemAvailable::<Impl, OFFSET>,
            MediaChange::<Impl, OFFSET>,
            CurrentMediaItemAvailable::<Impl, OFFSET>,
            CurrentItemChange::<Impl, OFFSET>,
            MediaCollectionChange::<Impl, OFFSET>,
            MediaCollectionAttributeStringAdded::<Impl, OFFSET>,
            MediaCollectionAttributeStringRemoved::<Impl, OFFSET>,
            MediaCollectionAttributeStringChanged::<Impl, OFFSET>,
            PlaylistCollectionChange::<Impl, OFFSET>,
            PlaylistCollectionPlaylistAdded::<Impl, OFFSET>,
            PlaylistCollectionPlaylistRemoved::<Impl, OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted::<Impl, OFFSET>,
            ModeChange::<Impl, OFFSET>,
            MediaError::<Impl, OFFSET>,
            OpenPlaylistSwitch::<Impl, OFFSET>,
            DomainChange::<Impl, OFFSET>,
            SwitchedToPlayerApplication::<Impl, OFFSET>,
            SwitchedToControl::<Impl, OFFSET>,
            PlayerDockedStateChange::<Impl, OFFSET>,
            PlayerReconnect::<Impl, OFFSET>,
            Click::<Impl, OFFSET>,
            DoubleClick::<Impl, OFFSET>,
            KeyDown::<Impl, OFFSET>,
            KeyPress::<Impl, OFFSET>,
            KeyUp::<Impl, OFFSET>,
            MouseDown::<Impl, OFFSET>,
            MouseMove::<Impl, OFFSET>,
            MouseUp::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPEvents2Impl: Sized + IWMPEventsImpl {
    fn DeviceConnect();
    fn DeviceDisconnect();
    fn DeviceStatusChange();
    fn DeviceSyncStateChange();
    fn DeviceSyncError();
    fn CreatePartnershipComplete();
}
impl ::windows::core::RuntimeName for IWMPEvents2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEvents2";
}
impl IWMPEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2Impl, const OFFSET: isize>() -> IWMPEvents2Vtbl {
        unsafe extern "system" fn DeviceConnect<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceConnect(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceDisconnect<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceDisconnect(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceStatusChange<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstatus: WMPDeviceStatus) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceStatusChange(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), newstatus).into()
        }
        unsafe extern "system" fn DeviceSyncStateChange<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstate: WMPSyncState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSyncStateChange(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), newstate).into()
        }
        unsafe extern "system" fn DeviceSyncError<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSyncError(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), &*(&pmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreatePartnershipComplete<Impl: IWMPEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePartnershipComplete(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), hrresult).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPEvents2>, ::windows::core::GetTrustLevel, DeviceConnect::<Impl, OFFSET>, DeviceDisconnect::<Impl, OFFSET>, DeviceStatusChange::<Impl, OFFSET>, DeviceSyncStateChange::<Impl, OFFSET>, DeviceSyncError::<Impl, OFFSET>, CreatePartnershipComplete::<Impl, OFFSET>)
    }
}
pub trait IWMPEvents3Impl: Sized + IWMPEvents2Impl + IWMPEventsImpl {
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
impl ::windows::core::RuntimeName for IWMPEvents3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEvents3";
}
impl IWMPEvents3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3Impl, const OFFSET: isize>() -> IWMPEvents3Vtbl {
        unsafe extern "system" fn CdromRipStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, wmprs: WMPRipState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromRipStateChange(&*(&pcdromrip as *const <IWMPCdromRip as ::windows::core::Abi>::Abi as *const <IWMPCdromRip as ::windows::core::DefaultType>::DefaultType), wmprs).into()
        }
        unsafe extern "system" fn CdromRipMediaError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromRipMediaError(&*(&pcdromrip as *const <IWMPCdromRip as ::windows::core::Abi>::Abi as *const <IWMPCdromRip as ::windows::core::DefaultType>::DefaultType), &*(&pmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CdromBurnStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, wmpbs: WMPBurnState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnStateChange(&*(&pcdromburn as *const <IWMPCdromBurn as ::windows::core::Abi>::Abi as *const <IWMPCdromBurn as ::windows::core::DefaultType>::DefaultType), wmpbs).into()
        }
        unsafe extern "system" fn CdromBurnMediaError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnMediaError(&*(&pcdromburn as *const <IWMPCdromBurn as ::windows::core::Abi>::Abi as *const <IWMPCdromBurn as ::windows::core::DefaultType>::DefaultType), &*(&pmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CdromBurnError<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnError(&*(&pcdromburn as *const <IWMPCdromBurn as ::windows::core::Abi>::Abi as *const <IWMPCdromBurn as ::windows::core::DefaultType>::DefaultType), hrerror).into()
        }
        unsafe extern "system" fn LibraryConnect<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LibraryConnect(&*(&plibrary as *const <IWMPLibrary as ::windows::core::Abi>::Abi as *const <IWMPLibrary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LibraryDisconnect<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LibraryDisconnect(&*(&plibrary as *const <IWMPLibrary as ::windows::core::Abi>::Abi as *const <IWMPLibrary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FolderScanStateChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderScanStateChange(wmpfss).into()
        }
        unsafe extern "system" fn StringCollectionChange<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispstringcollection: ::windows::core::RawPtr, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StringCollectionChange(&*(&pdispstringcollection as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), change, lcollectionindex).into()
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionMediaAdded(&*(&pdispmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Impl: IWMPEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionMediaRemoved(&*(&pdispmedia as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPEvents3>,
            ::windows::core::GetTrustLevel,
            CdromRipStateChange::<Impl, OFFSET>,
            CdromRipMediaError::<Impl, OFFSET>,
            CdromBurnStateChange::<Impl, OFFSET>,
            CdromBurnMediaError::<Impl, OFFSET>,
            CdromBurnError::<Impl, OFFSET>,
            LibraryConnect::<Impl, OFFSET>,
            LibraryDisconnect::<Impl, OFFSET>,
            FolderScanStateChange::<Impl, OFFSET>,
            StringCollectionChange::<Impl, OFFSET>,
            MediaCollectionMediaAdded::<Impl, OFFSET>,
            MediaCollectionMediaRemoved::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPEvents4Impl: Sized + IWMPEvents3Impl + IWMPEvents2Impl + IWMPEventsImpl {
    fn DeviceEstimation();
}
impl ::windows::core::RuntimeName for IWMPEvents4 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPEvents4";
}
impl IWMPEvents4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents4Impl, const OFFSET: isize>() -> IWMPEvents4Vtbl {
        unsafe extern "system" fn DeviceEstimation<Impl: IWMPEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceEstimation(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), hrresult, qwestimatedusedspace, qwestimatedspace).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPEvents4>, ::windows::core::GetTrustLevel, DeviceEstimation::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPFolderMonitorServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPFolderMonitorServices";
}
impl IWMPFolderMonitorServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>() -> IWMPFolderMonitorServicesVtbl {
        unsafe extern "system" fn count<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(lindex, &*(&pbstrfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn add<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).add(&*(&bstrfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).remove(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scanState<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).scanState(pwmpfss) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentFolder<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentFolder(&*(&pbstrfolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scannedFilesCount<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).scannedFilesCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addedFilesCount<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).addedFilesCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn updateProgress<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).updateProgress(plprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startScan<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startScan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopScan<Impl: IWMPFolderMonitorServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stopScan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPFolderMonitorServices>,
            ::windows::core::GetTrustLevel,
            count::<Impl, OFFSET>,
            item::<Impl, OFFSET>,
            add::<Impl, OFFSET>,
            remove::<Impl, OFFSET>,
            scanState::<Impl, OFFSET>,
            currentFolder::<Impl, OFFSET>,
            scannedFilesCount::<Impl, OFFSET>,
            addedFilesCount::<Impl, OFFSET>,
            updateProgress::<Impl, OFFSET>,
            startScan::<Impl, OFFSET>,
            stopScan::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPGraphCreationImpl: Sized {
    fn GraphCreationPreRender();
    fn GraphCreationPostRender();
    fn GetGraphCreationFlags();
}
impl ::windows::core::RuntimeName for IWMPGraphCreation {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPGraphCreation";
}
impl IWMPGraphCreationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreationImpl, const OFFSET: isize>() -> IWMPGraphCreationVtbl {
        unsafe extern "system" fn GraphCreationPreRender<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphCreationPreRender(&*(&pfiltergraph as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&preserved as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GraphCreationPostRender<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphCreationPostRender(&*(&pfiltergraph as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGraphCreationFlags<Impl: IWMPGraphCreationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGraphCreationFlags(pdwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPGraphCreation>, ::windows::core::GetTrustLevel, GraphCreationPreRender::<Impl, OFFSET>, GraphCreationPostRender::<Impl, OFFSET>, GetGraphCreationFlags::<Impl, OFFSET>)
    }
}
pub trait IWMPLibraryImpl: Sized {
    fn name();
    fn r#type();
    fn mediaCollection();
    fn isIdentical();
}
impl ::windows::core::RuntimeName for IWMPLibrary {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPLibrary";
}
impl IWMPLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryImpl, const OFFSET: isize>() -> IWMPLibraryVtbl {
        unsafe extern "system" fn name<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).name(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).r#type(pwmplt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mediaCollection(::core::mem::transmute_copy(&ppiwmpmediacollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmplibrary: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isIdentical(&*(&piwmplibrary as *const <IWMPLibrary as ::windows::core::Abi>::Abi as *const <IWMPLibrary as ::windows::core::DefaultType>::DefaultType), pvbool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPLibrary>, ::windows::core::GetTrustLevel, name::<Impl, OFFSET>, r#type::<Impl, OFFSET>, mediaCollection::<Impl, OFFSET>, isIdentical::<Impl, OFFSET>)
    }
}
pub trait IWMPLibrary2Impl: Sized + IWMPLibraryImpl {
    fn getItemInfo();
}
impl ::windows::core::RuntimeName for IWMPLibrary2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPLibrary2";
}
impl IWMPLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary2Impl, const OFFSET: isize>() -> IWMPLibrary2Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPLibrary2>, ::windows::core::GetTrustLevel, getItemInfo::<Impl, OFFSET>)
    }
}
pub trait IWMPLibraryServicesImpl: Sized {
    fn getCountByType();
    fn getLibraryByType();
}
impl ::windows::core::RuntimeName for IWMPLibraryServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPLibraryServices";
}
impl IWMPLibraryServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServicesImpl, const OFFSET: isize>() -> IWMPLibraryServicesVtbl {
        unsafe extern "system" fn getCountByType<Impl: IWMPLibraryServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getCountByType(wmplt, plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLibraryByType<Impl: IWMPLibraryServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLibraryByType(wmplt, lindex, ::core::mem::transmute_copy(&ppiwmplibrary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPLibraryServices>, ::windows::core::GetTrustLevel, getCountByType::<Impl, OFFSET>, getLibraryByType::<Impl, OFFSET>)
    }
}
pub trait IWMPLibrarySharingServicesImpl: Sized {
    fn isLibraryShared();
    fn isLibrarySharingEnabled();
    fn showLibrarySharing();
}
impl ::windows::core::RuntimeName for IWMPLibrarySharingServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPLibrarySharingServices";
}
impl IWMPLibrarySharingServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>() -> IWMPLibrarySharingServicesVtbl {
        unsafe extern "system" fn isLibraryShared<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isLibraryShared(pvbshared) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isLibrarySharingEnabled(pvbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn showLibrarySharing<Impl: IWMPLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).showLibrarySharing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPLibrarySharingServices>, ::windows::core::GetTrustLevel, isLibraryShared::<Impl, OFFSET>, isLibrarySharingEnabled::<Impl, OFFSET>, showLibrarySharing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMedia {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMedia";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaImpl, const OFFSET: isize>() -> IWMPMediaVtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isIdentical(&*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType), pvbool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceURL<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).sourceURL(&*(&pbstrsourceurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn name<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).name(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setname<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setname(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn imageSourceWidth<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).imageSourceWidth(pwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn imageSourceHeight<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).imageSourceHeight(pheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn markerCount<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).markerCount(pmarkercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMarkerTime<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getMarkerTime(markernum, pmarkertime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMarkerName<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getMarkerName(markernum, &*(&pbstrmarkername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn duration<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).duration(pduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn durationString<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).durationString(&*(&pbstrduration as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributeCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttributeName<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeName(lindex, &*(&pbstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfoByAtom<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfoByAtom(latom, &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isMemberOf<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr, pvarfismemberof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isMemberOf(&*(&pplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), pvarfismemberof) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReadOnlyItem<Impl: IWMPMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfisreadonly: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isReadOnlyItem(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pvarfisreadonly) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPMedia>,
            ::windows::core::GetTrustLevel,
            isIdentical::<Impl, OFFSET>,
            sourceURL::<Impl, OFFSET>,
            name::<Impl, OFFSET>,
            Setname::<Impl, OFFSET>,
            imageSourceWidth::<Impl, OFFSET>,
            imageSourceHeight::<Impl, OFFSET>,
            markerCount::<Impl, OFFSET>,
            getMarkerTime::<Impl, OFFSET>,
            getMarkerName::<Impl, OFFSET>,
            duration::<Impl, OFFSET>,
            durationString::<Impl, OFFSET>,
            attributeCount::<Impl, OFFSET>,
            getAttributeName::<Impl, OFFSET>,
            getItemInfo::<Impl, OFFSET>,
            setItemInfo::<Impl, OFFSET>,
            getItemInfoByAtom::<Impl, OFFSET>,
            isMemberOf::<Impl, OFFSET>,
            isReadOnlyItem::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia2Impl: Sized + IWMPMediaImpl + IDispatchImpl {
    fn error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMedia2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMedia2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia2Impl, const OFFSET: isize>() -> IWMPMedia2Vtbl {
        unsafe extern "system" fn error<Impl: IWMPMedia2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).error(::core::mem::transmute_copy(&ppiwmperroritem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMedia2>, ::windows::core::GetTrustLevel, error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia3Impl: Sized + IWMPMedia2Impl + IWMPMediaImpl + IDispatchImpl {
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMedia3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMedia3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3Impl, const OFFSET: isize>() -> IWMPMedia3Vtbl {
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPMedia3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeCountByType(&*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrlanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPMedia3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfoByType(
                &*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrlanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lindex,
                &*(&pvarvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMedia3>, ::windows::core::GetTrustLevel, getAttributeCountByType::<Impl, OFFSET>, getItemInfoByType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMediaCollection {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMediaCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollectionImpl, const OFFSET: isize>() -> IWMPMediaCollectionVtbl {
        unsafe extern "system" fn add<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).add(&*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAll(::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByGenre<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgenre: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByGenre(&*(&bstrgenre as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAuthor<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAuthor(&*(&bstrauthor as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAlbum<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstralbum: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAlbum(&*(&bstralbum as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttribute<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAttribute(&*(&bstrattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmediaitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfdeletefile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).remove(&*(&pitem as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType), varfdeletefile) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttributeStringCollection<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeStringCollection(&*(&bstrattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrmediatype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstringcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMediaAtom<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getMediaAtom(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), platom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setDeleted(&*(&pitem as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType), varfisdeleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPMediaCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isDeleted(&*(&pitem as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType), pvarfisdeleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPMediaCollection>,
            ::windows::core::GetTrustLevel,
            add::<Impl, OFFSET>,
            getAll::<Impl, OFFSET>,
            getByName::<Impl, OFFSET>,
            getByGenre::<Impl, OFFSET>,
            getByAuthor::<Impl, OFFSET>,
            getByAlbum::<Impl, OFFSET>,
            getByAttribute::<Impl, OFFSET>,
            remove::<Impl, OFFSET>,
            getAttributeStringCollection::<Impl, OFFSET>,
            getMediaAtom::<Impl, OFFSET>,
            setDeleted::<Impl, OFFSET>,
            isDeleted::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaCollection2Impl: Sized + IWMPMediaCollectionImpl + IDispatchImpl {
    fn createQuery();
    fn getPlaylistByQuery();
    fn getStringCollectionByQuery();
    fn getByAttributeAndMediaType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMediaCollection2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMediaCollection2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2Impl, const OFFSET: isize>() -> IWMPMediaCollection2Vtbl {
        unsafe extern "system" fn createQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createQuery(::core::mem::transmute_copy(&ppquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getPlaylistByQuery(
                &*(&pquery as *const <IWMPQuery as ::windows::core::Abi>::Abi as *const <IWMPQuery as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrmediatype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrsortattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                fsortascending,
                ::core::mem::transmute_copy(&ppplaylist),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getStringCollectionByQuery(
                &*(&bstrattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pquery as *const <IWMPQuery as ::windows::core::Abi>::Abi as *const <IWMPQuery as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrmediatype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrsortattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                fsortascending,
                ::core::mem::transmute_copy(&ppstringcollection),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Impl: IWMPMediaCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAttributeAndMediaType(
                &*(&bstrattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrmediatype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmediaitems),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMediaCollection2>, ::windows::core::GetTrustLevel, createQuery::<Impl, OFFSET>, getPlaylistByQuery::<Impl, OFFSET>, getStringCollectionByQuery::<Impl, OFFSET>, getByAttributeAndMediaType::<Impl, OFFSET>)
    }
}
pub trait IWMPMediaPluginRegistrarImpl: Sized {
    fn WMPRegisterPlayerPlugin();
    fn WMPUnRegisterPlayerPlugin();
}
impl ::windows::core::RuntimeName for IWMPMediaPluginRegistrar {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMediaPluginRegistrar";
}
impl IWMPMediaPluginRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrarImpl, const OFFSET: isize>() -> IWMPMediaPluginRegistrarVtbl {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WMPRegisterPlayerPlugin(
                &*(&pwszfriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszuninstallstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwpriority,
                &*(&guidplugintype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cmediatypes,
                &*(&pmediatypes as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WMPUnRegisterPlayerPlugin(&*(&guidplugintype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMediaPluginRegistrar>, ::windows::core::GetTrustLevel, WMPRegisterPlayerPlugin::<Impl, OFFSET>, WMPUnRegisterPlayerPlugin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataPictureImpl: Sized + IDispatchImpl {
    fn mimeType();
    fn pictureType();
    fn description();
    fn URL();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMetadataPicture {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMetadataPicture";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataPictureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPictureImpl, const OFFSET: isize>() -> IWMPMetadataPictureVtbl {
        unsafe extern "system" fn mimeType<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mimeType(&*(&pbstrmimetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pictureType<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pictureType(&*(&pbstrpicturetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn description<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).description(&*(&pbstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IWMPMetadataPictureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).URL(&*(&pbstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMetadataPicture>, ::windows::core::GetTrustLevel, mimeType::<Impl, OFFSET>, pictureType::<Impl, OFFSET>, description::<Impl, OFFSET>, URL::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataTextImpl: Sized + IDispatchImpl {
    fn description();
    fn text();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPMetadataText {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPMetadataText";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataTextImpl, const OFFSET: isize>() -> IWMPMetadataTextVtbl {
        unsafe extern "system" fn description<Impl: IWMPMetadataTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).description(&*(&pbstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IWMPMetadataTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).text(&*(&pbstrtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPMetadataText>, ::windows::core::GetTrustLevel, description::<Impl, OFFSET>, text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPNetwork {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNetwork";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetworkImpl, const OFFSET: isize>() -> IWMPNetworkVtbl {
        unsafe extern "system" fn bandWidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).bandWidth(plbandwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn recoveredPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).recoveredPackets(plrecoveredpackets) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceProtocol<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).sourceProtocol(&*(&pbstrsourceprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn receivedPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).receivedPackets(plreceivedpackets) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lostPackets<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lostPackets(pllostpackets) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn receptionQuality<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).receptionQuality(plreceptionquality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bufferingCount<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).bufferingCount(plbufferingcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bufferingProgress<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).bufferingProgress(plbufferingprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bufferingTime<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).bufferingTime(plbufferingtime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbufferingTime<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetbufferingTime(lbufferingtime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frameRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).frameRate(plframerate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxBitRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxBitRate(plbitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bitRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).bitRate(plbitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProxySettings<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProxySettings(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), plproxysetting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxySettings<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setProxySettings(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lproxysetting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProxyName<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProxyName(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrproxyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxyName<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setProxyName(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrproxyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProxyPort<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProxyPort(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lproxyport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxyPort<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setProxyPort(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lproxyport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProxyExceptionList<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProxyExceptionList(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrexceptionlist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxyExceptionList<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setProxyExceptionList(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrexceptionlist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProxyBypassForLocal<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfbypassforlocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getProxyBypassForLocal(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pfbypassforlocal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxyBypassForLocal<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fbypassforlocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setProxyBypassForLocal(&*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fbypassforlocal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxBandwidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).maxBandwidth(lmaxbandwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetmaxBandwidth<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetmaxBandwidth(lmaxbandwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn downloadProgress<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).downloadProgress(pldownloadprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn encodedFrameRate<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).encodedFrameRate(plframerate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn framesSkipped<Impl: IWMPNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).framesSkipped(plframes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPNetwork>,
            ::windows::core::GetTrustLevel,
            bandWidth::<Impl, OFFSET>,
            recoveredPackets::<Impl, OFFSET>,
            sourceProtocol::<Impl, OFFSET>,
            receivedPackets::<Impl, OFFSET>,
            lostPackets::<Impl, OFFSET>,
            receptionQuality::<Impl, OFFSET>,
            bufferingCount::<Impl, OFFSET>,
            bufferingProgress::<Impl, OFFSET>,
            bufferingTime::<Impl, OFFSET>,
            SetbufferingTime::<Impl, OFFSET>,
            frameRate::<Impl, OFFSET>,
            maxBitRate::<Impl, OFFSET>,
            bitRate::<Impl, OFFSET>,
            getProxySettings::<Impl, OFFSET>,
            setProxySettings::<Impl, OFFSET>,
            getProxyName::<Impl, OFFSET>,
            setProxyName::<Impl, OFFSET>,
            getProxyPort::<Impl, OFFSET>,
            setProxyPort::<Impl, OFFSET>,
            getProxyExceptionList::<Impl, OFFSET>,
            setProxyExceptionList::<Impl, OFFSET>,
            getProxyBypassForLocal::<Impl, OFFSET>,
            setProxyBypassForLocal::<Impl, OFFSET>,
            maxBandwidth::<Impl, OFFSET>,
            SetmaxBandwidth::<Impl, OFFSET>,
            downloadProgress::<Impl, OFFSET>,
            encodedFrameRate::<Impl, OFFSET>,
            framesSkipped::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPNodeRealEstateImpl: Sized {
    fn GetDesiredSize();
    fn SetRects();
    fn GetRects();
    fn SetWindowless();
    fn GetWindowless();
    fn SetFullScreen();
    fn GetFullScreen();
}
impl ::windows::core::RuntimeName for IWMPNodeRealEstate {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeRealEstate";
}
impl IWMPNodeRealEstateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>() -> IWMPNodeRealEstateVtbl {
        unsafe extern "system" fn GetDesiredSize<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesiredSize(&*(&psize as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRects<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRects(
                &*(&psrc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdest as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pclip as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRects<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRects(
                &*(&psrc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pdest as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pclip as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowless<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowless(&*(&fwindowless as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowless<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowless(&*(&pfwindowless as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullScreen<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFullScreen(&*(&ffullscreen as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullScreen<Impl: IWMPNodeRealEstateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullScreen(&*(&pffullscreen as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeRealEstate>, ::windows::core::GetTrustLevel, GetDesiredSize::<Impl, OFFSET>, SetRects::<Impl, OFFSET>, GetRects::<Impl, OFFSET>, SetWindowless::<Impl, OFFSET>, GetWindowless::<Impl, OFFSET>, SetFullScreen::<Impl, OFFSET>, GetFullScreen::<Impl, OFFSET>)
    }
}
pub trait IWMPNodeRealEstateHostImpl: Sized {
    fn OnDesiredSizeChange();
    fn OnFullScreenTransition();
}
impl ::windows::core::RuntimeName for IWMPNodeRealEstateHost {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeRealEstateHost";
}
impl IWMPNodeRealEstateHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHostImpl, const OFFSET: isize>() -> IWMPNodeRealEstateHostVtbl {
        unsafe extern "system" fn OnDesiredSizeChange<Impl: IWMPNodeRealEstateHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDesiredSizeChange(&*(&psize as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFullScreenTransition<Impl: IWMPNodeRealEstateHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnFullScreenTransition(&*(&ffullscreen as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeRealEstateHost>, ::windows::core::GetTrustLevel, OnDesiredSizeChange::<Impl, OFFSET>, OnFullScreenTransition::<Impl, OFFSET>)
    }
}
pub trait IWMPNodeWindowedImpl: Sized {
    fn SetOwnerWindow();
    fn GetOwnerWindow();
}
impl ::windows::core::RuntimeName for IWMPNodeWindowed {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeWindowed";
}
impl IWMPNodeWindowedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedImpl, const OFFSET: isize>() -> IWMPNodeWindowedVtbl {
        unsafe extern "system" fn SetOwnerWindow<Impl: IWMPNodeWindowedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOwnerWindow(hwnd) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerWindow<Impl: IWMPNodeWindowedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerWindow(phwnd) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeWindowed>, ::windows::core::GetTrustLevel, SetOwnerWindow::<Impl, OFFSET>, GetOwnerWindow::<Impl, OFFSET>)
    }
}
pub trait IWMPNodeWindowedHostImpl: Sized {
    fn OnWindowMessageFromRenderer();
}
impl ::windows::core::RuntimeName for IWMPNodeWindowedHost {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeWindowedHost";
}
impl IWMPNodeWindowedHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedHostImpl, const OFFSET: isize>() -> IWMPNodeWindowedHostVtbl {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Impl: IWMPNodeWindowedHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowMessageFromRenderer(
                umsg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&plret as *const <super::super::Foundation::LRESULT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LRESULT as ::windows::core::DefaultType>::DefaultType),
                &*(&pfhandled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeWindowedHost>, ::windows::core::GetTrustLevel, OnWindowMessageFromRenderer::<Impl, OFFSET>)
    }
}
pub trait IWMPNodeWindowlessImpl: Sized + IWMPWindowMessageSinkImpl {
    fn OnDraw();
}
impl ::windows::core::RuntimeName for IWMPNodeWindowless {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeWindowless";
}
impl IWMPNodeWindowlessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessImpl, const OFFSET: isize>() -> IWMPNodeWindowlessVtbl {
        unsafe extern "system" fn OnDraw<Impl: IWMPNodeWindowlessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDraw(hdc, &*(&prcdraw as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeWindowless>, ::windows::core::GetTrustLevel, OnDraw::<Impl, OFFSET>)
    }
}
pub trait IWMPNodeWindowlessHostImpl: Sized {
    fn InvalidateRect();
}
impl ::windows::core::RuntimeName for IWMPNodeWindowlessHost {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPNodeWindowlessHost";
}
impl IWMPNodeWindowlessHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessHostImpl, const OFFSET: isize>() -> IWMPNodeWindowlessHostVtbl {
        unsafe extern "system" fn InvalidateRect<Impl: IWMPNodeWindowlessHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateRect(&*(&prc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&ferase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPNodeWindowlessHost>, ::windows::core::GetTrustLevel, InvalidateRect::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayerImpl: Sized + IWMPCoreImpl + IDispatchImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlayer {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayer";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerImpl, const OFFSET: isize>() -> IWMPPlayerVtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enabled(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setenabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fullScreen(pbfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetfullScreen(bfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enableContextMenu(pbenablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetenableContextMenu(benablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuiMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).uiMode(&*(&pbstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPPlayer>,
            ::windows::core::GetTrustLevel,
            enabled::<Impl, OFFSET>,
            Setenabled::<Impl, OFFSET>,
            fullScreen::<Impl, OFFSET>,
            SetfullScreen::<Impl, OFFSET>,
            enableContextMenu::<Impl, OFFSET>,
            SetenableContextMenu::<Impl, OFFSET>,
            SetuiMode::<Impl, OFFSET>,
            uiMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer2Impl: Sized + IWMPCoreImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlayer2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayer2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2Impl, const OFFSET: isize>() -> IWMPPlayer2Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enabled(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setenabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fullScreen(pbfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetfullScreen(bfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enableContextMenu(pbenablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetenableContextMenu(benablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuiMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).uiMode(&*(&pbstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stretchToFit(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetstretchToFit(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).windowlessVideo(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetwindowlessVideo(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPPlayer2>,
            ::windows::core::GetTrustLevel,
            enabled::<Impl, OFFSET>,
            Setenabled::<Impl, OFFSET>,
            fullScreen::<Impl, OFFSET>,
            SetfullScreen::<Impl, OFFSET>,
            enableContextMenu::<Impl, OFFSET>,
            SetenableContextMenu::<Impl, OFFSET>,
            SetuiMode::<Impl, OFFSET>,
            uiMode::<Impl, OFFSET>,
            stretchToFit::<Impl, OFFSET>,
            SetstretchToFit::<Impl, OFFSET>,
            windowlessVideo::<Impl, OFFSET>,
            SetwindowlessVideo::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer3Impl: Sized + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlayer3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayer3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3Impl, const OFFSET: isize>() -> IWMPPlayer3Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enabled(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setenabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fullScreen(pbfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetfullScreen(bfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enableContextMenu(pbenablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetenableContextMenu(benablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuiMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).uiMode(&*(&pbstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stretchToFit(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetstretchToFit(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).windowlessVideo(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetwindowlessVideo(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPPlayer3>,
            ::windows::core::GetTrustLevel,
            enabled::<Impl, OFFSET>,
            Setenabled::<Impl, OFFSET>,
            fullScreen::<Impl, OFFSET>,
            SetfullScreen::<Impl, OFFSET>,
            enableContextMenu::<Impl, OFFSET>,
            SetenableContextMenu::<Impl, OFFSET>,
            SetuiMode::<Impl, OFFSET>,
            uiMode::<Impl, OFFSET>,
            stretchToFit::<Impl, OFFSET>,
            SetstretchToFit::<Impl, OFFSET>,
            windowlessVideo::<Impl, OFFSET>,
            SetwindowlessVideo::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer4Impl: Sized + IWMPCore3Impl + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlayer4 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayer4";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4Impl, const OFFSET: isize>() -> IWMPPlayer4Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enabled(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setenabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).fullScreen(pbfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetfullScreen(bfullscreen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enableContextMenu(pbenablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetenableContextMenu(benablecontextmenu) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuiMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).uiMode(&*(&pbstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stretchToFit(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetstretchToFit(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).windowlessVideo(pbenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetwindowlessVideo(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isRemote<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isRemote(pvarfisremote) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playerApplication<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playerApplication(::core::mem::transmute_copy(&ppiwmpplayerapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openPlayer<Impl: IWMPPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).openPlayer(&*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPPlayer4>,
            ::windows::core::GetTrustLevel,
            enabled::<Impl, OFFSET>,
            Setenabled::<Impl, OFFSET>,
            fullScreen::<Impl, OFFSET>,
            SetfullScreen::<Impl, OFFSET>,
            enableContextMenu::<Impl, OFFSET>,
            SetenableContextMenu::<Impl, OFFSET>,
            SetuiMode::<Impl, OFFSET>,
            uiMode::<Impl, OFFSET>,
            stretchToFit::<Impl, OFFSET>,
            SetstretchToFit::<Impl, OFFSET>,
            windowlessVideo::<Impl, OFFSET>,
            SetwindowlessVideo::<Impl, OFFSET>,
            isRemote::<Impl, OFFSET>,
            playerApplication::<Impl, OFFSET>,
            openPlayer::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayerApplicationImpl: Sized + IDispatchImpl {
    fn switchToPlayerApplication();
    fn switchToControl();
    fn playerDocked();
    fn hasDisplay();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlayerApplication {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayerApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayerApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>() -> IWMPPlayerApplicationVtbl {
        unsafe extern "system" fn switchToPlayerApplication<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).switchToPlayerApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn switchToControl<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).switchToControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playerDocked<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playerDocked(pbplayerdocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasDisplay<Impl: IWMPPlayerApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).hasDisplay(pbhasdisplay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlayerApplication>, ::windows::core::GetTrustLevel, switchToPlayerApplication::<Impl, OFFSET>, switchToControl::<Impl, OFFSET>, playerDocked::<Impl, OFFSET>, hasDisplay::<Impl, OFFSET>)
    }
}
pub trait IWMPPlayerServicesImpl: Sized {
    fn activateUIPlugin();
    fn setTaskPane();
    fn setTaskPaneURL();
}
impl ::windows::core::RuntimeName for IWMPPlayerServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayerServices";
}
impl IWMPPlayerServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServicesImpl, const OFFSET: isize>() -> IWMPPlayerServicesVtbl {
        unsafe extern "system" fn activateUIPlugin<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).activateUIPlugin(&*(&bstrplugin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setTaskPane<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setTaskPane(&*(&bstrtaskpane as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setTaskPaneURL<Impl: IWMPPlayerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setTaskPaneURL(
                &*(&bstrtaskpane as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlayerServices>, ::windows::core::GetTrustLevel, activateUIPlugin::<Impl, OFFSET>, setTaskPane::<Impl, OFFSET>, setTaskPaneURL::<Impl, OFFSET>)
    }
}
pub trait IWMPPlayerServices2Impl: Sized + IWMPPlayerServicesImpl {
    fn setBackgroundProcessingPriority();
}
impl ::windows::core::RuntimeName for IWMPPlayerServices2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlayerServices2";
}
impl IWMPPlayerServices2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices2Impl, const OFFSET: isize>() -> IWMPPlayerServices2Vtbl {
        unsafe extern "system" fn setBackgroundProcessingPriority<Impl: IWMPPlayerServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpriority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setBackgroundProcessingPriority(&*(&bstrpriority as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlayerServices2>, ::windows::core::GetTrustLevel, setBackgroundProcessingPriority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlaylist {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlaylist";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistImpl, const OFFSET: isize>() -> IWMPPlaylistVtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn name<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).name(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setname<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setname(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributeCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeName<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attributeName(lindex, &*(&pbstrattributename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(lindex, ::core::mem::transmute_copy(&ppiwmpmedia)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setItemInfo(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpplaylist: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isIdentical(&*(&piwmpplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), pvbool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn clear<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).insertItem(lindex, &*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).appendItem(&*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).removeItem(&*(&piwmpmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn moveItem<Impl: IWMPPlaylistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).moveItem(lindexold, lindexnew) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPPlaylist>,
            ::windows::core::GetTrustLevel,
            count::<Impl, OFFSET>,
            name::<Impl, OFFSET>,
            Setname::<Impl, OFFSET>,
            attributeCount::<Impl, OFFSET>,
            attributeName::<Impl, OFFSET>,
            item::<Impl, OFFSET>,
            getItemInfo::<Impl, OFFSET>,
            setItemInfo::<Impl, OFFSET>,
            isIdentical::<Impl, OFFSET>,
            clear::<Impl, OFFSET>,
            insertItem::<Impl, OFFSET>,
            appendItem::<Impl, OFFSET>,
            removeItem::<Impl, OFFSET>,
            moveItem::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistArrayImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlaylistArray {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlaylistArray";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArrayImpl, const OFFSET: isize>() -> IWMPPlaylistArrayVtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylistArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylistArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(lindex, ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlaylistArray>, ::windows::core::GetTrustLevel, count::<Impl, OFFSET>, item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistCollectionImpl: Sized + IDispatchImpl {
    fn newPlaylist();
    fn getAll();
    fn getByName();
    fn remove();
    fn setDeleted();
    fn isDeleted();
    fn importPlaylist();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPPlaylistCollection {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlaylistCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>() -> IWMPPlaylistCollectionVtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newPlaylist(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAll(::core::mem::transmute_copy(&ppplaylistarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppplaylistarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).remove(&*(&pitem as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setDeleted(&*(&pitem as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), varfisdeleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isDeleted(&*(&pitem as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), pvarfisdeleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importPlaylist<Impl: IWMPPlaylistCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, ppimporteditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).importPlaylist(&*(&pitem as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppimporteditem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlaylistCollection>, ::windows::core::GetTrustLevel, newPlaylist::<Impl, OFFSET>, getAll::<Impl, OFFSET>, getByName::<Impl, OFFSET>, remove::<Impl, OFFSET>, setDeleted::<Impl, OFFSET>, isDeleted::<Impl, OFFSET>, importPlaylist::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IWMPPlugin {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPlugin";
}
impl IWMPPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginImpl, const OFFSET: isize>() -> IWMPPluginVtbl {
        unsafe extern "system" fn Init<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(dwplaybackcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetID<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps(pdwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseWMPServices<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpservices: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseWMPServices(&*(&pwmpservices as *const <IWMPServices as ::windows::core::Abi>::Abi as *const <IWMPServices as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdviseWMPServices<Impl: IWMPPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnAdviseWMPServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPlugin>, ::windows::core::GetTrustLevel, Init::<Impl, OFFSET>, Shutdown::<Impl, OFFSET>, GetID::<Impl, OFFSET>, GetCaps::<Impl, OFFSET>, AdviseWMPServices::<Impl, OFFSET>, UnAdviseWMPServices::<Impl, OFFSET>)
    }
}
pub trait IWMPPluginEnableImpl: Sized {
    fn SetEnable();
    fn GetEnable();
}
impl ::windows::core::RuntimeName for IWMPPluginEnable {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPluginEnable";
}
impl IWMPPluginEnableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnableImpl, const OFFSET: isize>() -> IWMPPluginEnableVtbl {
        unsafe extern "system" fn SetEnable<Impl: IWMPPluginEnableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnable(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnable<Impl: IWMPPluginEnableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnable(&*(&pfenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPluginEnable>, ::windows::core::GetTrustLevel, SetEnable::<Impl, OFFSET>, GetEnable::<Impl, OFFSET>)
    }
}
pub trait IWMPPluginUIImpl: Sized {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn DisplayPropertyPage();
    fn GetProperty();
    fn SetProperty();
    fn TranslateAccelerator();
}
impl ::windows::core::RuntimeName for IWMPPluginUI {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPPluginUI";
}
impl IWMPPluginUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUIImpl, const OFFSET: isize>() -> IWMPPluginUIVtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCore(&*(&pcore as *const <IWMPCore as ::windows::core::Abi>::Abi as *const <IWMPCore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&phwndwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPropertyPage(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvarproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvarproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IWMPPluginUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAccelerator(&*(&lpmsg as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPPluginUI>, ::windows::core::GetTrustLevel, SetCore::<Impl, OFFSET>, Create::<Impl, OFFSET>, Destroy::<Impl, OFFSET>, DisplayPropertyPage::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, TranslateAccelerator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPQueryImpl: Sized + IDispatchImpl {
    fn addCondition();
    fn beginNextGroup();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPQuery {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPQuery";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQueryImpl, const OFFSET: isize>() -> IWMPQueryVtbl {
        unsafe extern "system" fn addCondition<Impl: IWMPQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroperator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).addCondition(
                &*(&bstrattribute as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstroperator as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn beginNextGroup<Impl: IWMPQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).beginNextGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPQuery>, ::windows::core::GetTrustLevel, addCondition::<Impl, OFFSET>, beginNextGroup::<Impl, OFFSET>)
    }
}
pub trait IWMPRemoteMediaServicesImpl: Sized {
    fn GetServiceType();
    fn GetApplicationName();
    fn GetScriptableObject();
    fn GetCustomUIMode();
}
impl ::windows::core::RuntimeName for IWMPRemoteMediaServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPRemoteMediaServices";
}
impl IWMPRemoteMediaServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>() -> IWMPRemoteMediaServicesVtbl {
        unsafe extern "system" fn GetServiceType<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceType(&*(&pbstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationName<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationName(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptableObject<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScriptableObject(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdispatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomUIMode<Impl: IWMPRemoteMediaServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomUIMode(&*(&pbstrfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPRemoteMediaServices>, ::windows::core::GetTrustLevel, GetServiceType::<Impl, OFFSET>, GetApplicationName::<Impl, OFFSET>, GetScriptableObject::<Impl, OFFSET>, GetCustomUIMode::<Impl, OFFSET>)
    }
}
pub trait IWMPRenderConfigImpl: Sized {
    fn SetinProcOnly();
    fn inProcOnly();
}
impl ::windows::core::RuntimeName for IWMPRenderConfig {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPRenderConfig";
}
impl IWMPRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfigImpl, const OFFSET: isize>() -> IWMPRenderConfigVtbl {
        unsafe extern "system" fn SetinProcOnly<Impl: IWMPRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetinProcOnly(&*(&finproc as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn inProcOnly<Impl: IWMPRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).inProcOnly(&*(&pfinproc as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPRenderConfig>, ::windows::core::GetTrustLevel, SetinProcOnly::<Impl, OFFSET>, inProcOnly::<Impl, OFFSET>)
    }
}
pub trait IWMPServicesImpl: Sized {
    fn GetStreamTime();
    fn GetStreamState();
}
impl ::windows::core::RuntimeName for IWMPServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPServices";
}
impl IWMPServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServicesImpl, const OFFSET: isize>() -> IWMPServicesVtbl {
        unsafe extern "system" fn GetStreamTime<Impl: IWMPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamTime(prt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamState<Impl: IWMPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamState(pstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPServices>, ::windows::core::GetTrustLevel, GetStreamTime::<Impl, OFFSET>, GetStreamState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPSettings {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSettings";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettingsImpl, const OFFSET: isize>() -> IWMPSettingsVtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isAvailable(&*(&bstritem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pisavailable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn autoStart<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautostart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).autoStart(pfautostart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetautoStart<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fautostart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetautoStart(fautostart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseURL<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).baseURL(&*(&pbstrbaseurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetbaseURL(&*(&bstrbaseurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultFrame<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).defaultFrame(&*(&pbstrdefaultframe as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdefaultFrame<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetdefaultFrame(&*(&bstrdefaultframe as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn invokeURLs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).invokeURLs(pfinvokeurls) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetinvokeURLs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finvokeurls: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetinvokeURLs(finvokeurls) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mute<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mute(pfmute) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setmute<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setmute(fmute) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playCount<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetplayCount<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetplayCount(lcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn rate<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).rate(pdrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setrate<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setrate(drate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn balance<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).balance(plbalance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setbalance<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setbalance(lbalance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn volume<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).volume(plvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setvolume<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setvolume(lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMode<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pvarfmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setMode<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setMode(&*(&bstrmode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), varfmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enableErrorDialogs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).enableErrorDialogs(pfenableerrordialogs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetenableErrorDialogs<Impl: IWMPSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetenableErrorDialogs(fenableerrordialogs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPSettings>,
            ::windows::core::GetTrustLevel,
            isAvailable::<Impl, OFFSET>,
            autoStart::<Impl, OFFSET>,
            SetautoStart::<Impl, OFFSET>,
            baseURL::<Impl, OFFSET>,
            SetbaseURL::<Impl, OFFSET>,
            defaultFrame::<Impl, OFFSET>,
            SetdefaultFrame::<Impl, OFFSET>,
            invokeURLs::<Impl, OFFSET>,
            SetinvokeURLs::<Impl, OFFSET>,
            mute::<Impl, OFFSET>,
            Setmute::<Impl, OFFSET>,
            playCount::<Impl, OFFSET>,
            SetplayCount::<Impl, OFFSET>,
            rate::<Impl, OFFSET>,
            Setrate::<Impl, OFFSET>,
            balance::<Impl, OFFSET>,
            Setbalance::<Impl, OFFSET>,
            volume::<Impl, OFFSET>,
            Setvolume::<Impl, OFFSET>,
            getMode::<Impl, OFFSET>,
            setMode::<Impl, OFFSET>,
            enableErrorDialogs::<Impl, OFFSET>,
            SetenableErrorDialogs::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSettings2Impl: Sized + IWMPSettingsImpl + IDispatchImpl {
    fn defaultAudioLanguage();
    fn mediaAccessRights();
    fn requestMediaAccessRights();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPSettings2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSettings2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2Impl, const OFFSET: isize>() -> IWMPSettings2Vtbl {
        unsafe extern "system" fn defaultAudioLanguage<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).defaultAudioLanguage(pllangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mediaAccessRights<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mediaAccessRights(&*(&pbstrrights as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn requestMediaAccessRights<Impl: IWMPSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvbaccepted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).requestMediaAccessRights(&*(&bstrdesiredaccess as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pvbaccepted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSettings2>, ::windows::core::GetTrustLevel, defaultAudioLanguage::<Impl, OFFSET>, mediaAccessRights::<Impl, OFFSET>, requestMediaAccessRights::<Impl, OFFSET>)
    }
}
pub trait IWMPSkinManagerImpl: Sized {
    fn SetVisualStyle();
}
impl ::windows::core::RuntimeName for IWMPSkinManager {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSkinManager";
}
impl IWMPSkinManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSkinManagerImpl, const OFFSET: isize>() -> IWMPSkinManagerVtbl {
        unsafe extern "system" fn SetVisualStyle<Impl: IWMPSkinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVisualStyle(&*(&bstrpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSkinManager>, ::windows::core::GetTrustLevel, SetVisualStyle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPStringCollection {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPStringCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollectionImpl, const OFFSET: isize>() -> IWMPStringCollectionVtbl {
        unsafe extern "system" fn count<Impl: IWMPStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IWMPStringCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(lindex, &*(&pbstrstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPStringCollection>, ::windows::core::GetTrustLevel, count::<Impl, OFFSET>, item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollection2Impl: Sized + IWMPStringCollectionImpl + IDispatchImpl {
    fn isIdentical();
    fn getItemInfo();
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPStringCollection2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPStringCollection2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2Impl, const OFFSET: isize>() -> IWMPStringCollection2Vtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isIdentical(&*(&piwmpstringcollection2 as *const <IWMPStringCollection2 as ::windows::core::Abi>::Abi as *const <IWMPStringCollection2 as ::windows::core::DefaultType>::DefaultType), pvbool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(lcollectionindex, &*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeCountByType(lcollectionindex, &*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrlanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPStringCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfoByType(
                lcollectionindex,
                &*(&bstrtype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrlanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lattributeindex,
                &*(&pvarvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPStringCollection2>, ::windows::core::GetTrustLevel, isIdentical::<Impl, OFFSET>, getItemInfo::<Impl, OFFSET>, getAttributeCountByType::<Impl, OFFSET>, getItemInfoByType::<Impl, OFFSET>)
    }
}
pub trait IWMPSubscriptionServiceImpl: Sized {
    fn allowPlay();
    fn allowCDBurn();
    fn allowPDATransfer();
    fn startBackgroundProcessing();
}
impl ::windows::core::RuntimeName for IWMPSubscriptionService {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSubscriptionService";
}
impl IWMPSubscriptionServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>() -> IWMPSubscriptionServiceVtbl {
        unsafe extern "system" fn allowPlay<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: ::windows::core::RawPtr, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowPlay(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pmedia as *const <IWMPMedia as ::windows::core::Abi>::Abi as *const <IWMPMedia as ::windows::core::DefaultType>::DefaultType), &*(&pfallowplay as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allowCDBurn<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowCDBurn(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType),
                &*(&pfallowburn as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allowPDATransfer<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowPDATransfer(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType),
                &*(&pfallowtransfer as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startBackgroundProcessing<Impl: IWMPSubscriptionServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startBackgroundProcessing(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSubscriptionService>, ::windows::core::GetTrustLevel, allowPlay::<Impl, OFFSET>, allowCDBurn::<Impl, OFFSET>, allowPDATransfer::<Impl, OFFSET>, startBackgroundProcessing::<Impl, OFFSET>)
    }
}
pub trait IWMPSubscriptionService2Impl: Sized + IWMPSubscriptionServiceImpl {
    fn stopBackgroundProcessing();
    fn serviceEvent();
    fn deviceAvailable();
    fn prepareForSync();
}
impl ::windows::core::RuntimeName for IWMPSubscriptionService2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSubscriptionService2";
}
impl IWMPSubscriptionService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>() -> IWMPSubscriptionService2Vtbl {
        unsafe extern "system" fn stopBackgroundProcessing<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stopBackgroundProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn serviceEvent<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).serviceEvent(event) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deviceAvailable<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).deviceAvailable(&*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcb as *const <IWMPSubscriptionServiceCallback as ::windows::core::Abi>::Abi as *const <IWMPSubscriptionServiceCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prepareForSync<Impl: IWMPSubscriptionService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).prepareForSync(
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcb as *const <IWMPSubscriptionServiceCallback as ::windows::core::Abi>::Abi as *const <IWMPSubscriptionServiceCallback as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSubscriptionService2>, ::windows::core::GetTrustLevel, stopBackgroundProcessing::<Impl, OFFSET>, serviceEvent::<Impl, OFFSET>, deviceAvailable::<Impl, OFFSET>, prepareForSync::<Impl, OFFSET>)
    }
}
pub trait IWMPSubscriptionServiceCallbackImpl: Sized {
    fn onComplete();
}
impl ::windows::core::RuntimeName for IWMPSubscriptionServiceCallback {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSubscriptionServiceCallback";
}
impl IWMPSubscriptionServiceCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceCallbackImpl, const OFFSET: isize>() -> IWMPSubscriptionServiceCallbackVtbl {
        unsafe extern "system" fn onComplete<Impl: IWMPSubscriptionServiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).onComplete(hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSubscriptionServiceCallback>, ::windows::core::GetTrustLevel, onComplete::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IWMPSyncDevice {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSyncDevice";
}
impl IWMPSyncDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDeviceImpl, const OFFSET: isize>() -> IWMPSyncDeviceVtbl {
        unsafe extern "system" fn friendlyName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).friendlyName(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetfriendlyName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetfriendlyName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deviceName<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).deviceName(&*(&pbstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deviceId<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).deviceId(&*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn partnershipIndex<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).partnershipIndex(plindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn connected<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).connected(pvbconnected) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).status(pwmpds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn syncState<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).syncState(pwmpss) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn progress<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).progress(plprogress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createPartnership<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbshowui: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createPartnership(vbshowui) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deletePartnership<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).deletePartnership() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn start<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stop<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn showSettings<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).showSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPSyncDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).isIdentical(&*(&pdevice as *const <IWMPSyncDevice as ::windows::core::Abi>::Abi as *const <IWMPSyncDevice as ::windows::core::DefaultType>::DefaultType), pvbool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWMPSyncDevice>,
            ::windows::core::GetTrustLevel,
            friendlyName::<Impl, OFFSET>,
            SetfriendlyName::<Impl, OFFSET>,
            deviceName::<Impl, OFFSET>,
            deviceId::<Impl, OFFSET>,
            partnershipIndex::<Impl, OFFSET>,
            connected::<Impl, OFFSET>,
            status::<Impl, OFFSET>,
            syncState::<Impl, OFFSET>,
            progress::<Impl, OFFSET>,
            getItemInfo::<Impl, OFFSET>,
            createPartnership::<Impl, OFFSET>,
            deletePartnership::<Impl, OFFSET>,
            start::<Impl, OFFSET>,
            stop::<Impl, OFFSET>,
            showSettings::<Impl, OFFSET>,
            isIdentical::<Impl, OFFSET>,
        )
    }
}
pub trait IWMPSyncDevice2Impl: Sized + IWMPSyncDeviceImpl {
    fn setItemInfo();
}
impl ::windows::core::RuntimeName for IWMPSyncDevice2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSyncDevice2";
}
impl IWMPSyncDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice2Impl, const OFFSET: isize>() -> IWMPSyncDevice2Vtbl {
        unsafe extern "system" fn setItemInfo<Impl: IWMPSyncDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setItemInfo(&*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSyncDevice2>, ::windows::core::GetTrustLevel, setItemInfo::<Impl, OFFSET>)
    }
}
pub trait IWMPSyncDevice3Impl: Sized + IWMPSyncDevice2Impl + IWMPSyncDeviceImpl {
    fn estimateSyncSize();
    fn cancelEstimation();
}
impl ::windows::core::RuntimeName for IWMPSyncDevice3 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSyncDevice3";
}
impl IWMPSyncDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3Impl, const OFFSET: isize>() -> IWMPSyncDevice3Vtbl {
        unsafe extern "system" fn estimateSyncSize<Impl: IWMPSyncDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: ::windows::core::RawPtr, prulesplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).estimateSyncSize(&*(&pnonruleplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType), &*(&prulesplaylist as *const <IWMPPlaylist as ::windows::core::Abi>::Abi as *const <IWMPPlaylist as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cancelEstimation<Impl: IWMPSyncDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).cancelEstimation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSyncDevice3>, ::windows::core::GetTrustLevel, estimateSyncSize::<Impl, OFFSET>, cancelEstimation::<Impl, OFFSET>)
    }
}
pub trait IWMPSyncServicesImpl: Sized {
    fn deviceCount();
    fn getDevice();
}
impl ::windows::core::RuntimeName for IWMPSyncServices {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPSyncServices";
}
impl IWMPSyncServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServicesImpl, const OFFSET: isize>() -> IWMPSyncServicesVtbl {
        unsafe extern "system" fn deviceCount<Impl: IWMPSyncServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).deviceCount(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDevice<Impl: IWMPSyncServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDevice(lindex, ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPSyncServices>, ::windows::core::GetTrustLevel, deviceCount::<Impl, OFFSET>, getDevice::<Impl, OFFSET>)
    }
}
pub trait IWMPTranscodePolicyImpl: Sized {
    fn allowTranscode();
}
impl ::windows::core::RuntimeName for IWMPTranscodePolicy {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPTranscodePolicy";
}
impl IWMPTranscodePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPTranscodePolicyImpl, const OFFSET: isize>() -> IWMPTranscodePolicyVtbl {
        unsafe extern "system" fn allowTranscode<Impl: IWMPTranscodePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvballow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowTranscode(pvballow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPTranscodePolicy>, ::windows::core::GetTrustLevel, allowTranscode::<Impl, OFFSET>)
    }
}
pub trait IWMPUserEventSinkImpl: Sized {
    fn NotifyUserEvent();
}
impl ::windows::core::RuntimeName for IWMPUserEventSink {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPUserEventSink";
}
impl IWMPUserEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPUserEventSinkImpl, const OFFSET: isize>() -> IWMPUserEventSinkVtbl {
        unsafe extern "system" fn NotifyUserEvent<Impl: IWMPUserEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyUserEvent(eventcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPUserEventSink>, ::windows::core::GetTrustLevel, NotifyUserEvent::<Impl, OFFSET>)
    }
}
pub trait IWMPVideoRenderConfigImpl: Sized {
    fn SetpresenterActivate();
}
impl ::windows::core::RuntimeName for IWMPVideoRenderConfig {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPVideoRenderConfig";
}
impl IWMPVideoRenderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPVideoRenderConfigImpl, const OFFSET: isize>() -> IWMPVideoRenderConfigVtbl {
        unsafe extern "system" fn SetpresenterActivate<Impl: IWMPVideoRenderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetpresenterActivate(&*(&pactivate as *const <super::MediaFoundation::IMFActivate as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFActivate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPVideoRenderConfig>, ::windows::core::GetTrustLevel, SetpresenterActivate::<Impl, OFFSET>)
    }
}
pub trait IWMPWindowMessageSinkImpl: Sized {
    fn OnWindowMessage();
}
impl ::windows::core::RuntimeName for IWMPWindowMessageSink {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IWMPWindowMessageSink";
}
impl IWMPWindowMessageSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPWindowMessageSinkImpl, const OFFSET: isize>() -> IWMPWindowMessageSinkVtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPWindowMessageSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowMessage(
                umsg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&plret as *const <super::super::Foundation::LRESULT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LRESULT as ::windows::core::DefaultType>::DefaultType),
                &*(&pfhandled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMPWindowMessageSink>, ::windows::core::GetTrustLevel, OnWindowMessage::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXFeed {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeed";
}
impl IXFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedImpl, const OFFSET: isize>() -> IXFeedVtbl {
        unsafe extern "system" fn Xml<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(uiitemcount, sortproperty, sortorder, filterflags, includeflags, ::core::mem::transmute_copy(&pps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUrl(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWriteTime(::core::mem::transmute_copy(&pstlastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncSetting<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncSetting(::core::mem::transmute_copy(&pfss)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSyncSetting(fss) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interval<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval(::core::mem::transmute_copy(&puiinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterval(uiinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime(::core::mem::transmute_copy(&pstlastdownloadtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalEnclosurePath(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items(::core::mem::transmute_copy(&ppfe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(uiid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAllItemsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxItemCount(::core::mem::transmute_copy(&puimaxitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxItemCount(uimaxitemcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically(::core::mem::transmute_copy(&pbdownloadenclosuresautomatically)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDownloadEnclosuresAutomatically(&*(&bdownloadenclosuresautomatically as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus(::core::mem::transmute_copy(&pfds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError(::core::mem::transmute_copy(&pfde)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Merge(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&ppsztitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&ppszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&ppszhomepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image(::core::mem::transmute_copy(&ppszimageurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBuildDate(::core::mem::transmute_copy(&pstlastbuilddate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate(::core::mem::transmute_copy(&pstpubdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ttl(::core::mem::transmute_copy(&puittl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language(::core::mem::transmute_copy(&ppszlanguage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copyright(::core::mem::transmute_copy(&ppszcopyright)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsList(::core::mem::transmute_copy(&pbislist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(scope, mask, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadItemCount(::core::mem::transmute_copy(&puiunreaditemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IXFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount(::core::mem::transmute_copy(&puiitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeed>,
            ::windows::core::GetTrustLevel,
            Xml::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Rename::<Impl, OFFSET>,
            Url::<Impl, OFFSET>,
            SetUrl::<Impl, OFFSET>,
            LocalId::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            LastWriteTime::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Download::<Impl, OFFSET>,
            AsyncDownload::<Impl, OFFSET>,
            CancelAsyncDownload::<Impl, OFFSET>,
            SyncSetting::<Impl, OFFSET>,
            SetSyncSetting::<Impl, OFFSET>,
            Interval::<Impl, OFFSET>,
            SetInterval::<Impl, OFFSET>,
            LastDownloadTime::<Impl, OFFSET>,
            LocalEnclosurePath::<Impl, OFFSET>,
            Items::<Impl, OFFSET>,
            GetItem::<Impl, OFFSET>,
            MarkAllItemsRead::<Impl, OFFSET>,
            MaxItemCount::<Impl, OFFSET>,
            SetMaxItemCount::<Impl, OFFSET>,
            DownloadEnclosuresAutomatically::<Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically::<Impl, OFFSET>,
            DownloadStatus::<Impl, OFFSET>,
            LastDownloadError::<Impl, OFFSET>,
            Merge::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            Link::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            LastBuildDate::<Impl, OFFSET>,
            PubDate::<Impl, OFFSET>,
            Ttl::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            Copyright::<Impl, OFFSET>,
            IsList::<Impl, OFFSET>,
            GetWatcher::<Impl, OFFSET>,
            UnreadItemCount::<Impl, OFFSET>,
            ItemCount::<Impl, OFFSET>,
        )
    }
}
pub trait IXFeed2Impl: Sized + IXFeedImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
impl ::windows::core::RuntimeName for IXFeed2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeed2";
}
impl IXFeed2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2Impl, const OFFSET: isize>() -> IXFeed2Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByEffectiveId(uieffectiveid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastItemDownloadTime(::core::mem::transmute_copy(&pstlastitemdownloadtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username(::core::mem::transmute_copy(&ppszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password(::core::mem::transmute_copy(&ppszpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCredentials(&*(&pszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszpassword as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCredentials<Impl: IXFeed2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXFeed2>, ::windows::core::GetTrustLevel, GetItemByEffectiveId::<Impl, OFFSET>, LastItemDownloadTime::<Impl, OFFSET>, Username::<Impl, OFFSET>, Password::<Impl, OFFSET>, SetCredentials::<Impl, OFFSET>, ClearCredentials::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXFeedEnclosure {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedEnclosure";
}
impl IXFeedEnclosureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosureImpl, const OFFSET: isize>() -> IXFeedEnclosureVtbl {
        unsafe extern "system" fn Url<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ppszmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&puilength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAsyncDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus(::core::mem::transmute_copy(&pfds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError(::core::mem::transmute_copy(&pfde)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPath(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMimeType(::core::mem::transmute_copy(&ppszmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFile<Impl: IXFeedEnclosureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdownloadurl: super::super::Foundation::PWSTR, pszdownloadfilepath: super::super::Foundation::PWSTR, pszdownloadmimetype: super::super::Foundation::PWSTR, pszenclosurefilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFile(
                &*(&pszdownloadurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdownloadfilepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdownloadmimetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszenclosurefilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedEnclosure>,
            ::windows::core::GetTrustLevel,
            Url::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Length::<Impl, OFFSET>,
            AsyncDownload::<Impl, OFFSET>,
            CancelAsyncDownload::<Impl, OFFSET>,
            DownloadStatus::<Impl, OFFSET>,
            LastDownloadError::<Impl, OFFSET>,
            LocalPath::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            DownloadMimeType::<Impl, OFFSET>,
            RemoveFile::<Impl, OFFSET>,
            SetFile::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXFeedEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedEvents";
}
impl IXFeedEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEventsImpl, const OFFSET: isize>() -> IXFeedEventsVtbl {
        unsafe extern "system" fn Error<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDeleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedRenamed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedUrlChanged(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMoved<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMoved(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloading(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloadCompleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fde) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedItemCountChanged(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), feicfflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedEvents>,
            ::windows::core::GetTrustLevel,
            Error::<Impl, OFFSET>,
            FeedDeleted::<Impl, OFFSET>,
            FeedRenamed::<Impl, OFFSET>,
            FeedUrlChanged::<Impl, OFFSET>,
            FeedMoved::<Impl, OFFSET>,
            FeedDownloading::<Impl, OFFSET>,
            FeedDownloadCompleted::<Impl, OFFSET>,
            FeedItemCountChanged::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXFeedFolder {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedFolder";
}
impl IXFeedFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderImpl, const OFFSET: isize>() -> IXFeedFolderVtbl {
        unsafe extern "system" fn Feeds<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Feeds(::core::mem::transmute_copy(&ppfe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subfolders(::core::mem::transmute_copy(&ppfe)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFeed(
                &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubfolder(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbfeedexists as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsSubfolder(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbsubfolderexists as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubfolder(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoot(::core::mem::transmute_copy(&pbisrootfeedfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(scope, mask, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalUnreadItemCount(::core::mem::transmute_copy(&puitotalunreaditemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Impl: IXFeedFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalItemCount(::core::mem::transmute_copy(&puitotalitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedFolder>,
            ::windows::core::GetTrustLevel,
            Feeds::<Impl, OFFSET>,
            Subfolders::<Impl, OFFSET>,
            CreateFeed::<Impl, OFFSET>,
            CreateSubfolder::<Impl, OFFSET>,
            ExistsFeed::<Impl, OFFSET>,
            ExistsSubfolder::<Impl, OFFSET>,
            GetFeed::<Impl, OFFSET>,
            GetSubfolder::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Rename::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            IsRoot::<Impl, OFFSET>,
            GetWatcher::<Impl, OFFSET>,
            TotalUnreadItemCount::<Impl, OFFSET>,
            TotalItemCount::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXFeedFolderEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedFolderEvents";
}
impl IXFeedFolderEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEventsImpl, const OFFSET: isize>() -> IXFeedFolderEventsVtbl {
        unsafe extern "system" fn Error<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderAdded<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderAdded(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderDeleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderDeleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderRenamed<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderRenamed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderMovedFrom(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderMovedTo(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderItemCountChanged(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), feicfflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedAdded<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedAdded(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDeleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedRenamed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedUrlChanged(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMovedFrom(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedMovedTo(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloading(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedDownloadCompleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fde) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedFolderEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeedItemCountChanged(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), feicfflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedFolderEvents>,
            ::windows::core::GetTrustLevel,
            Error::<Impl, OFFSET>,
            FolderAdded::<Impl, OFFSET>,
            FolderDeleted::<Impl, OFFSET>,
            FolderRenamed::<Impl, OFFSET>,
            FolderMovedFrom::<Impl, OFFSET>,
            FolderMovedTo::<Impl, OFFSET>,
            FolderItemCountChanged::<Impl, OFFSET>,
            FeedAdded::<Impl, OFFSET>,
            FeedDeleted::<Impl, OFFSET>,
            FeedRenamed::<Impl, OFFSET>,
            FeedUrlChanged::<Impl, OFFSET>,
            FeedMovedFrom::<Impl, OFFSET>,
            FeedMovedTo::<Impl, OFFSET>,
            FeedDownloading::<Impl, OFFSET>,
            FeedDownloadCompleted::<Impl, OFFSET>,
            FeedItemCountChanged::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXFeedItem {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedItem";
}
impl IXFeedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItemImpl, const OFFSET: isize>() -> IXFeedItemVtbl {
        unsafe extern "system" fn Xml<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(fxif, ::core::mem::transmute_copy(&pps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&ppsztitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszguid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid(::core::mem::transmute_copy(&ppszguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&ppszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate(::core::mem::transmute_copy(&pstpubdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comments(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author(::core::mem::transmute_copy(&ppszauthor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enclosure(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead(::core::mem::transmute_copy(&pbisread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsRead(&*(&bisread as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId(::core::mem::transmute_copy(&puiid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&ppszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime(::core::mem::transmute_copy(&pstlastdownloadtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: IXFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified(::core::mem::transmute_copy(&pstmodifiedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedItem>,
            ::windows::core::GetTrustLevel,
            Xml::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Link::<Impl, OFFSET>,
            Guid::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            PubDate::<Impl, OFFSET>,
            Comments::<Impl, OFFSET>,
            Author::<Impl, OFFSET>,
            Enclosure::<Impl, OFFSET>,
            IsRead::<Impl, OFFSET>,
            SetIsRead::<Impl, OFFSET>,
            LocalId::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            DownloadUrl::<Impl, OFFSET>,
            LastDownloadTime::<Impl, OFFSET>,
            Modified::<Impl, OFFSET>,
        )
    }
}
pub trait IXFeedItem2Impl: Sized + IXFeedItemImpl {
    fn EffectiveId();
}
impl ::windows::core::RuntimeName for IXFeedItem2 {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedItem2";
}
impl IXFeedItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem2Impl, const OFFSET: isize>() -> IXFeedItem2Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IXFeedItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveId(::core::mem::transmute_copy(&puieffectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXFeedItem2>, ::windows::core::GetTrustLevel, EffectiveId::<Impl, OFFSET>)
    }
}
pub trait IXFeedsEnumImpl: Sized {
    fn Count();
    fn Item();
}
impl ::windows::core::RuntimeName for IXFeedsEnum {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedsEnum";
}
impl IXFeedsEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnumImpl, const OFFSET: isize>() -> IXFeedsEnumVtbl {
        unsafe extern "system" fn Count<Impl: IXFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&puicount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IXFeedsEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(uiindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXFeedsEnum>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXFeedsManager {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer.IXFeedsManager";
}
impl IXFeedsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManagerImpl, const OFFSET: isize>() -> IXFeedsManagerVtbl {
        unsafe extern "system" fn RootFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootFolder(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscribed(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbsubscribed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbfeedexists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeedByUrl(&*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFolder(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbfolderexists)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolder(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFeed(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFolder(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSync<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSync(fbsa) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncStatus(::core::mem::transmute_copy(&pfbss)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInterval(::core::mem::transmute_copy(&puiinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultInterval(uiinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncSyncAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normalize<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamin: ::windows::core::RawPtr, ppstreamout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normalize(&*(&pstreamin as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstreamout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IXFeedsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCountLimit(::core::mem::transmute_copy(&puiitemcountlimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXFeedsManager>,
            ::windows::core::GetTrustLevel,
            RootFolder::<Impl, OFFSET>,
            IsSubscribed::<Impl, OFFSET>,
            ExistsFeed::<Impl, OFFSET>,
            GetFeed::<Impl, OFFSET>,
            GetFeedByUrl::<Impl, OFFSET>,
            ExistsFolder::<Impl, OFFSET>,
            GetFolder::<Impl, OFFSET>,
            DeleteFeed::<Impl, OFFSET>,
            DeleteFolder::<Impl, OFFSET>,
            BackgroundSync::<Impl, OFFSET>,
            BackgroundSyncStatus::<Impl, OFFSET>,
            DefaultInterval::<Impl, OFFSET>,
            SetDefaultInterval::<Impl, OFFSET>,
            AsyncSyncAll::<Impl, OFFSET>,
            Normalize::<Impl, OFFSET>,
            ItemCountLimit::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _WMPOCXEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _WMPOCXEvents {
    const NAME: &'static str = "Windows.Win32.Media.MediaPlayer._WMPOCXEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _WMPOCXEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _WMPOCXEventsImpl, const OFFSET: isize>() -> _WMPOCXEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_WMPOCXEvents>, ::windows::core::GetTrustLevel)
    }
}
