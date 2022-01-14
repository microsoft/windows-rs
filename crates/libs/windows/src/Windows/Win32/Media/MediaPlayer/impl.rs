#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&mut self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rename(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUrl(&mut self, feedurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, newparentpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn LastWriteTime(&mut self) -> ::windows::core::Result<f64>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Download(&mut self) -> ::windows::core::Result<()>;
    fn AsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn SyncSetting(&mut self) -> ::windows::core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&mut self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, minutes: i32) -> ::windows::core::Result<()>;
    fn LastDownloadTime(&mut self) -> ::windows::core::Result<f64>;
    fn LocalEnclosurePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetItem(&mut self, itemid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Link(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastBuildDate(&mut self) -> ::windows::core::Result<f64>;
    fn PubDate(&mut self) -> ::windows::core::Result<f64>;
    fn Ttl(&mut self) -> ::windows::core::Result<i32>;
    fn Language(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Copyright(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MaxItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxItemCount(&mut self, count: i32) -> ::windows::core::Result<()>;
    fn DownloadEnclosuresAutomatically(&mut self) -> ::windows::core::Result<i16>;
    fn SetDownloadEnclosuresAutomatically(&mut self, downloadenclosuresautomatically: i16) -> ::windows::core::Result<()>;
    fn DownloadStatus(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&mut self, feedxml: super::super::Foundation::BSTR, feedurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsList(&mut self) -> ::windows::core::Result<i16>;
    fn MarkAllItemsRead(&mut self) -> ::windows::core::Result<()>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn UnreadItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn ItemCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeed_Vtbl {
        unsafe extern "system" fn Xml<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Url<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *feedurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&feedurl)).into()
        }
        unsafe extern "system" fn LocalId<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *feedguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Download<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *syncsetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncSetting(::core::mem::transmute_copy(&syncsetting)).into()
        }
        unsafe extern "system" fn Interval<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&itemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homepage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *homepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *imageurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    *lastbuilddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *lastpopulatedate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    *ttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyright: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    *copyright = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxItemCount(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    *downloadenclosuresautomatically = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&downloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Merge(::core::mem::transmute_copy(&feedxml), ::core::mem::transmute_copy(&feedurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *feedurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsList() {
                ::core::result::Result::Ok(ok__) => {
                    *islist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarkAllItemsRead().into()
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeed2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFeed_Impl {
    fn GetItemByEffectiveId(&mut self, itemeffectiveid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn LastItemDownloadTime(&mut self) -> ::windows::core::Result<f64>;
    fn Username(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCredentials(&mut self, username: super::super::Foundation::BSTR, password: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClearCredentials(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByEffectiveId(::core::mem::transmute_copy(&itemeffectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastitemdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username() {
                ::core::result::Result::Ok(ok__) => {
                    *username = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *password = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn ClearCredentials<Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearCredentials().into()
        }
        Self {
            base: IFeed_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedEnclosure_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn AsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn DownloadStatus(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DownloadMimeType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RemoveFile(&mut self) -> ::windows::core::Result<()>;
    fn SetFile(&mut self, downloadurl: super::super::Foundation::BSTR, downloadfilepath: super::super::Foundation::BSTR, downloadmimetype: super::super::Foundation::BSTR, enclosurefilename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEnclosure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosureurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *mimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *localpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosureurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *mimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadmimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enclosurefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFile(::core::mem::transmute_copy(&downloadurl), ::core::mem::transmute_copy(&downloadfilepath), ::core::mem::transmute_copy(&downloadmimetype), ::core::mem::transmute_copy(&enclosurefilename)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMoved(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, path: super::super::Foundation::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, path: super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedMoved<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMoved(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedFolder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Feeds(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Subfolders(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateFeed(&mut self, feedname: super::super::Foundation::BSTR, feedurl: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateSubfolder(&mut self, foldername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFeed(&mut self, feedname: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFeed(&mut self, feedname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsSubfolder(&mut self, foldername: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetSubfolder(&mut self, foldername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rename(&mut self, foldername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, newparentpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsRoot(&mut self) -> ::windows::core::Result<i16>;
    fn TotalUnreadItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn TotalItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFeed(::core::mem::transmute_copy(&feedname), ::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(::core::mem::transmute_copy(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Name<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *foldername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&foldername)).into()
        }
        unsafe extern "system" fn Path<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *folderpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isroot: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *isroot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedFolderEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FolderAdded(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderDeleted(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderRenamed(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedFrom(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedTo(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderItemCountChanged(&mut self, path: super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
    fn FeedAdded(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedFrom(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedTo(&mut self, path: super::super::Foundation::BSTR, oldpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, path: super::super::Foundation::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, path: super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolderEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FolderAdded<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderAdded(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FolderDeleted<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FolderRenamed<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderMovedFrom(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderMovedTo(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        unsafe extern "system" fn FeedAdded<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedAdded(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMovedFrom(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMovedTo(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&mut self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Link(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Guid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PubDate(&mut self) -> ::windows::core::Result<f64>;
    fn Comments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Author(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enclosure(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsRead(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsRead(&mut self, isread: i16) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<i32>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastDownloadTime(&mut self) -> ::windows::core::Result<f64>;
    fn Modified(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linkurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *linkurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *itemguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comments() {
                ::core::result::Result::Ok(ok__) => {
                    *comments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *author = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enclosure() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *isread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRead(::core::mem::transmute_copy(&isread)).into()
        }
        unsafe extern "system" fn LocalId<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *itemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *itemurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *modified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFeedItem_Impl {
    fn EffectiveId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    *effectiveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFeedItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EffectiveId: EffectiveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsEnum_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnum_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFeedsManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RootFolder(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsSubscribed(&mut self, feedurl: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn ExistsFeed(&mut self, feedpath: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFeed(&mut self, feedpath: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetFeedByUrl(&mut self, feedurl: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFolder(&mut self, folderpath: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFolder(&mut self, folderpath: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DeleteFeed(&mut self, feedpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteFolder(&mut self, folderpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BackgroundSync(&mut self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()>;
    fn BackgroundSyncStatus(&mut self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultInterval(&mut self, minutes: i32) -> ::windows::core::Result<()>;
    fn AsyncSyncAll(&mut self) -> ::windows::core::Result<()>;
    fn Normalize(&mut self, feedxmlin: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ItemCountLimit(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subscribed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscribed(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *subscribed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeed(::core::mem::transmute_copy(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeedByUrl(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFolder(::core::mem::transmute_copy(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFeed(::core::mem::transmute_copy(&feedpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&folderpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackgroundSync(::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxmlin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedxmlout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normalize(::core::mem::transmute_copy(&feedxmlin)) {
                ::core::result::Result::Ok(ok__) => {
                    *feedxmlout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *itemcountlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPAudioRenderConfig_Impl: Sized {
    fn audioOutputDevice(&mut self, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetaudioOutputDevice(&mut self, bstroutputdevice: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPAudioRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPAudioRenderConfig_Vtbl {
        unsafe extern "system" fn audioOutputDevice<Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).audioOutputDevice(::core::mem::transmute_copy(&pbstroutputdevice)).into()
        }
        unsafe extern "system" fn SetaudioOutputDevice<Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetaudioOutputDevice(::core::mem::transmute_copy(&bstroutputdevice)).into()
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
pub trait IWMPCdrom_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn driveSpecifier(&mut self, pbstrdrive: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn playlist(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn eject(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdrom_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdrom_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdrom_Vtbl {
        unsafe extern "system" fn driveSpecifier<Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).driveSpecifier(::core::mem::transmute_copy(&pbstrdrive)).into()
        }
        unsafe extern "system" fn playlist<Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playlist() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eject<Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).eject().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPCdromBurn_Impl: Sized {
    fn isAvailable(&mut self, bstritem: super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritem: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn label(&mut self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setlabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn burnFormat(&mut self, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::Result<()>;
    fn SetburnFormat(&mut self, wmpbf: WMPBurnFormat) -> ::windows::core::Result<()>;
    fn burnPlaylist(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetburnPlaylist(&mut self, pplaylist: ::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn refreshStatus(&mut self) -> ::windows::core::Result<()>;
    fn burnState(&mut self, pwmpbs: *mut WMPBurnState) -> ::windows::core::Result<()>;
    fn burnProgress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startBurn(&mut self) -> ::windows::core::Result<()>;
    fn stopBurn(&mut self) -> ::windows::core::Result<()>;
    fn erase(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPCdromBurn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromBurn_Vtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn label<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).label(::core::mem::transmute_copy(&pbstrlabel)).into()
        }
        unsafe extern "system" fn Setlabel<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setlabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn burnFormat<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).burnFormat(::core::mem::transmute_copy(&pwmpbf)).into()
        }
        unsafe extern "system" fn SetburnFormat<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetburnFormat(::core::mem::transmute_copy(&wmpbf)).into()
        }
        unsafe extern "system" fn burnPlaylist<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).burnPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnPlaylist<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetburnPlaylist(::core::mem::transmute(&pplaylist)).into()
        }
        unsafe extern "system" fn refreshStatus<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).refreshStatus().into()
        }
        unsafe extern "system" fn burnState<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).burnState(::core::mem::transmute_copy(&pwmpbs)).into()
        }
        unsafe extern "system" fn burnProgress<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).burnProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startBurn<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startBurn().into()
        }
        unsafe extern "system" fn stopBurn<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stopBurn().into()
        }
        unsafe extern "system" fn erase<Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).erase().into()
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
pub trait IWMPCdromCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32) -> ::windows::core::Result<IWMPCdrom>;
    fn getByDriveSpecifier(&mut self, bstrdrivespecifier: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPCdrom>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdromCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromCollection_Vtbl {
        unsafe extern "system" fn count<Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcdrom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByDriveSpecifier(::core::mem::transmute_copy(&bstrdrivespecifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcdrom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWMPCdromRip_Impl: Sized {
    fn ripState(&mut self, pwmprs: *mut WMPRipState) -> ::windows::core::Result<()>;
    fn ripProgress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startRip(&mut self) -> ::windows::core::Result<()>;
    fn stopRip(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPCdromRip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCdromRip_Vtbl {
        unsafe extern "system" fn ripState<Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ripState(::core::mem::transmute_copy(&pwmprs)).into()
        }
        unsafe extern "system" fn ripProgress<Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ripProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startRip<Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startRip().into()
        }
        unsafe extern "system" fn stopRip<Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stopRip().into()
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
pub trait IWMPClosedCaption_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SAMIStyle(&mut self, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIStyle(&mut self, bstrsamistyle: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SAMILang(&mut self, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMILang(&mut self, bstrsamilang: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SAMIFileName(&mut self, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIFileName(&mut self, bstrsamifilename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn captioningId(&mut self, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetcaptioningId(&mut self, bstrcaptioningid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPClosedCaption_Vtbl {
        unsafe extern "system" fn SAMIStyle<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAMIStyle(::core::mem::transmute_copy(&pbstrsamistyle)).into()
        }
        unsafe extern "system" fn SetSAMIStyle<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSAMIStyle(::core::mem::transmute_copy(&bstrsamistyle)).into()
        }
        unsafe extern "system" fn SAMILang<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAMILang(::core::mem::transmute_copy(&pbstrsamilang)).into()
        }
        unsafe extern "system" fn SetSAMILang<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSAMILang(::core::mem::transmute_copy(&bstrsamilang)).into()
        }
        unsafe extern "system" fn SAMIFileName<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAMIFileName(::core::mem::transmute_copy(&pbstrsamifilename)).into()
        }
        unsafe extern "system" fn SetSAMIFileName<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSAMIFileName(::core::mem::transmute_copy(&bstrsamifilename)).into()
        }
        unsafe extern "system" fn captioningId<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).captioningId(::core::mem::transmute_copy(&pbstrcaptioningid)).into()
        }
        unsafe extern "system" fn SetcaptioningId<Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcaptioningId(::core::mem::transmute_copy(&bstrcaptioningid)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPClosedCaption2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPClosedCaption_Impl {
    fn SAMILangCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getSAMILangName(&mut self, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getSAMILangID(&mut self, nindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn SAMIStyleCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getSAMIStyleName(&mut self, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPClosedCaption2_Vtbl {
        unsafe extern "system" fn SAMILangCount<Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAMILangCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMILangName<Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getSAMILangName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn getSAMILangID<Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getSAMILangID(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SAMIStyleCount<Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAMIStyleCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMIStyleName<Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getSAMIStyleName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        Self {
            base: IWMPClosedCaption_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPContentContainer_Impl: Sized {
    fn GetID(&mut self) -> ::windows::core::Result<u32>;
    fn GetPrice(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetContentCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetContentPrice(&mut self, idxcontent: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetContentID(&mut self, idxcontent: u32) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPContentContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentContainer_Vtbl {
        unsafe extern "system" fn GetID<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrice<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrice() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentCount<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPrice<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentPrice(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentID<Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentID(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IWMPContentContainerList_Impl: Sized {
    fn GetTransactionType(&mut self) -> ::windows::core::Result<WMPTransactionType>;
    fn GetContainerCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetContainer(&mut self, idxcontainer: u32) -> ::windows::core::Result<IWMPContentContainer>;
}
impl IWMPContentContainerList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentContainerList_Vtbl {
        unsafe extern "system" fn GetTransactionType<Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pwmptt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerCount<Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainer(::core::mem::transmute_copy(&idxcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IWMPContentPartner_Impl: Sized {
    fn SetCallback(&mut self, pcallback: ::core::option::Option<IWMPContentPartnerCallback>) -> ::windows::core::Result<()>;
    fn Notify(&mut self, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetItemInfo(&mut self, bstrinfoname: super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetContentPartnerInfo(&mut self, bstrinfoname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCommands(&mut self, location: super::super::Foundation::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: super::super::Foundation::BSTR, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::Result<()>;
    fn InvokeCommand(&mut self, dwcommandid: u32, location: super::super::Foundation::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: super::super::Foundation::BSTR, citemids: u32, rgitemids: *const u32) -> ::windows::core::Result<()>;
    fn CanBuySilent(&mut self, pinfo: ::core::option::Option<IWMPContentContainerList>, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::Result<()>;
    fn Buy(&mut self, pinfo: ::core::option::Option<IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn GetStreamingURL(&mut self, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Download(&mut self, pinfo: ::core::option::Option<IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn DownloadTrackComplete(&mut self, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RefreshLicense(&mut self, dwcookie: u32, flocal: i16, bstrurl: super::super::Foundation::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: super::super::Foundation::BSTR, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCatalogURL(&mut self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetTemplate(&mut self, task: WMPTaskType, location: super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, clicklocation: super::super::Foundation::BSTR, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: super::super::Foundation::BSTR, bstrviewparams: super::super::Foundation::BSTR, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::Result<()>;
    fn UpdateDevice(&mut self, bstrdevicename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetListContents(&mut self, location: super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: super::super::Foundation::BSTR, bstrparams: super::super::Foundation::BSTR, dwlistcookie: u32) -> ::windows::core::Result<()>;
    fn Login(&mut self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::Result<()>;
    fn Authenticate(&mut self, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::Result<()>;
    fn Logout(&mut self) -> ::windows::core::Result<()>;
    fn SendMessage(&mut self, bstrmsg: super::super::Foundation::BSTR, bstrparam: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StationEvent(&mut self, bstrstationeventtype: super::super::Foundation::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: super::super::Foundation::BSTR, dwsecondsplayed: u32) -> ::windows::core::Result<()>;
    fn CompareContainerListPrices(&mut self, plistbase: ::core::option::Option<IWMPContentContainerList>, plistcompare: ::core::option::Option<IWMPContentContainerList>) -> ::windows::core::Result<i32>;
    fn VerifyPermission(&mut self, bstrpermission: super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentPartner_Vtbl {
        unsafe extern "system" fn SetCallback<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallback(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Notify<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn GetItemInfo<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemInfo(::core::mem::transmute_copy(&bstrinfoname), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPartnerInfo<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentPartnerInfo(::core::mem::transmute_copy(&bstrinfoname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCommands(::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute_copy(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&prgitemids), ::core::mem::transmute_copy(&pcitemids), ::core::mem::transmute_copy(&pprgitems)).into()
        }
        unsafe extern "system" fn InvokeCommand<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeCommand(::core::mem::transmute_copy(&dwcommandid), ::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute_copy(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&rgitemids)).into()
        }
        unsafe extern "system" fn CanBuySilent<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanBuySilent(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&pbstrtotalprice), ::core::mem::transmute_copy(&psilentok)).into()
        }
        unsafe extern "system" fn Buy<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Buy(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetStreamingURL<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamingURL(::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&pstreamcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn DownloadTrackComplete<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadTrackComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&downloadtrackparam)).into()
        }
        unsafe extern "system" fn RefreshLicense<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: i16, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshLicense(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&flocal), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&bstrrefreshreason), ::core::mem::transmute_copy(&preasoncontext)).into()
        }
        unsafe extern "system" fn GetCatalogURL<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCatalogURL(::core::mem::transmute_copy(&dwcatalogversion), ::core::mem::transmute_copy(&dwcatalogschemaversion), ::core::mem::transmute_copy(&cataloglcid), ::core::mem::transmute_copy(&pdwnewcatalogversion), ::core::mem::transmute_copy(&pbstrcatalogurl), ::core::mem::transmute_copy(&pexpirationdate)).into()
        }
        unsafe extern "system" fn GetTemplate<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, clicklocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrviewparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTemplate(::core::mem::transmute_copy(&task), ::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&clicklocation), ::core::mem::transmute_copy(&pclickcontext), ::core::mem::transmute_copy(&bstrfilter), ::core::mem::transmute_copy(&bstrviewparams), ::core::mem::transmute_copy(&pbstrtemplateurl), ::core::mem::transmute_copy(&ptemplatesize)).into()
        }
        unsafe extern "system" fn UpdateDevice<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDevice(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn GetListContents<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetListContents(::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&bstrlisttype), ::core::mem::transmute_copy(&bstrparams), ::core::mem::transmute_copy(&dwlistcookie)).into()
        }
        unsafe extern "system" fn Login<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Login(::core::mem::transmute_copy(&userinfo), ::core::mem::transmute_copy(&pwdinfo), ::core::mem::transmute_copy(&fusedcachedcreds), ::core::mem::transmute_copy(&foktocache)).into()
        }
        unsafe extern "system" fn Authenticate<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Authenticate(::core::mem::transmute_copy(&userinfo), ::core::mem::transmute_copy(&pwdinfo)).into()
        }
        unsafe extern "system" fn Logout<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logout().into()
        }
        unsafe extern "system" fn SendMessage<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(::core::mem::transmute_copy(&bstrmsg), ::core::mem::transmute_copy(&bstrparam)).into()
        }
        unsafe extern "system" fn StationEvent<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StationEvent(::core::mem::transmute_copy(&bstrstationeventtype), ::core::mem::transmute_copy(&stationid), ::core::mem::transmute_copy(&playlistindex), ::core::mem::transmute_copy(&trackid), ::core::mem::transmute_copy(&trackdata), ::core::mem::transmute_copy(&dwsecondsplayed)).into()
        }
        unsafe extern "system" fn CompareContainerListPrices<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistbase: ::windows::core::RawPtr, plistcompare: ::windows::core::RawPtr, presult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareContainerListPrices(::core::mem::transmute(&plistbase), ::core::mem::transmute(&plistcompare)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermission<Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VerifyPermission(::core::mem::transmute_copy(&bstrpermission), ::core::mem::transmute_copy(&pcontext)).into()
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
pub trait IWMPContentPartnerCallback_Impl: Sized {
    fn Notify(&mut self, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BuyComplete(&mut self, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::Result<()>;
    fn DownloadTrack(&mut self, cookie: u32, bstrtrackurl: super::super::Foundation::BSTR, dwservicetrackid: u32, bstrdownloadparams: super::super::Foundation::BSTR, hrdownload: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetCatalogVersion(&mut self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn UpdateDeviceComplete(&mut self, bstrdevicename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangeView(&mut self, bstrtype: super::super::Foundation::BSTR, bstrid: super::super::Foundation::BSTR, bstrfilter: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddListContents(&mut self, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::Result<()>;
    fn ListContentsComplete(&mut self, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn SendMessageComplete(&mut self, bstrmsg: super::super::Foundation::BSTR, bstrparam: super::super::Foundation::BSTR, bstrresult: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetContentIDsInLibrary(&mut self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::Result<()>;
    fn RefreshLicenseComplete(&mut self, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ShowPopup(&mut self, lindex: i32, bstrparameters: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn VerifyPermissionComplete(&mut self, bstrpermission: super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartnerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPContentPartnerCallback_Vtbl {
        unsafe extern "system" fn Notify<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn BuyComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuyComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwbuycookie)).into()
        }
        unsafe extern "system" fn DownloadTrack<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadTrack(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&bstrtrackurl), ::core::mem::transmute_copy(&dwservicetrackid), ::core::mem::transmute_copy(&bstrdownloadparams), ::core::mem::transmute_copy(&hrdownload)).into()
        }
        unsafe extern "system" fn GetCatalogVersion<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCatalogVersion(::core::mem::transmute_copy(&pdwversion), ::core::mem::transmute_copy(&pdwschemaversion), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn UpdateDeviceComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDeviceComplete(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn ChangeView<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeView(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrid), ::core::mem::transmute_copy(&bstrfilter)).into()
        }
        unsafe extern "system" fn AddListContents<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddListContents(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&citems), ::core::mem::transmute_copy(&prgitems)).into()
        }
        unsafe extern "system" fn ListContentsComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ListContentsComplete(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&hrsuccess)).into()
        }
        unsafe extern "system" fn SendMessageComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageComplete(::core::mem::transmute_copy(&bstrmsg), ::core::mem::transmute_copy(&bstrparam), ::core::mem::transmute_copy(&bstrresult)).into()
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentIDsInLibrary(::core::mem::transmute_copy(&pccontentids), ::core::mem::transmute_copy(&pprgids)).into()
        }
        unsafe extern "system" fn RefreshLicenseComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshLicenseComplete(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&hrrefresh)).into()
        }
        unsafe extern "system" fn ShowPopup<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPopup(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&bstrparameters)).into()
        }
        unsafe extern "system" fn VerifyPermissionComplete<Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VerifyPermissionComplete(::core::mem::transmute_copy(&bstrpermission), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrpermission)).into()
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
pub trait IWMPControls_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn play(&mut self) -> ::windows::core::Result<()>;
    fn stop(&mut self) -> ::windows::core::Result<()>;
    fn pause(&mut self) -> ::windows::core::Result<()>;
    fn fastForward(&mut self) -> ::windows::core::Result<()>;
    fn fastReverse(&mut self) -> ::windows::core::Result<()>;
    fn currentPosition(&mut self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()>;
    fn SetcurrentPosition(&mut self, dcurrentposition: f64) -> ::windows::core::Result<()>;
    fn currentPositionString(&mut self, pbstrcurrentposition: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn next(&mut self) -> ::windows::core::Result<()>;
    fn previous(&mut self) -> ::windows::core::Result<()>;
    fn currentItem(&mut self) -> ::windows::core::Result<IWMPMedia>;
    fn SetcurrentItem(&mut self, piwmpmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn currentMarker(&mut self, plmarker: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentMarker(&mut self, lmarker: i32) -> ::windows::core::Result<()>;
    fn playItem(&mut self, piwmpmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControls_Vtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn play<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).play().into()
        }
        unsafe extern "system" fn stop<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stop().into()
        }
        unsafe extern "system" fn pause<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pause().into()
        }
        unsafe extern "system" fn fastForward<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fastForward().into()
        }
        unsafe extern "system" fn fastReverse<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fastReverse().into()
        }
        unsafe extern "system" fn currentPosition<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentPosition(::core::mem::transmute_copy(&pdcurrentposition)).into()
        }
        unsafe extern "system" fn SetcurrentPosition<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentPosition(::core::mem::transmute_copy(&dcurrentposition)).into()
        }
        unsafe extern "system" fn currentPositionString<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentPositionString(::core::mem::transmute_copy(&pbstrcurrentposition)).into()
        }
        unsafe extern "system" fn next<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).next().into()
        }
        unsafe extern "system" fn previous<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).previous().into()
        }
        unsafe extern "system" fn currentItem<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentItem<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn currentMarker<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentMarker(::core::mem::transmute_copy(&plmarker)).into()
        }
        unsafe extern "system" fn SetcurrentMarker<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentMarker(::core::mem::transmute_copy(&lmarker)).into()
        }
        unsafe extern "system" fn playItem<Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).playItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPControls2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPControls_Impl {
    fn step(&mut self, lstep: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControls2_Vtbl {
        unsafe extern "system" fn step<Impl: IWMPControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).step(::core::mem::transmute_copy(&lstep)).into()
        }
        Self { base: IWMPControls_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), step: step::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPControls_Impl + IWMPControls2_Impl {
    fn audioLanguageCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getAudioLanguageID(&mut self, lindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn getAudioLanguageDescription(&mut self, lindex: i32, pbstrlangdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn currentAudioLanguage(&mut self, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentAudioLanguage(&mut self, llangid: i32) -> ::windows::core::Result<()>;
    fn currentAudioLanguageIndex(&mut self, plindex: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentAudioLanguageIndex(&mut self, lindex: i32) -> ::windows::core::Result<()>;
    fn getLanguageName(&mut self, llangid: i32, pbstrlangname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn currentPositionTimecode(&mut self, bstrtimecode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetcurrentPositionTimecode(&mut self, bstrtimecode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPControls3_Vtbl {
        unsafe extern "system" fn audioLanguageCount<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).audioLanguageCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAudioLanguageID<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAudioLanguageID(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn getAudioLanguageDescription<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAudioLanguageDescription(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrlangdesc)).into()
        }
        unsafe extern "system" fn currentAudioLanguage<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentAudioLanguage(::core::mem::transmute_copy(&llangid)).into()
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentAudioLanguageIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentAudioLanguageIndex(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn getLanguageName<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getLanguageName(::core::mem::transmute_copy(&llangid), ::core::mem::transmute_copy(&pbstrlangname)).into()
        }
        unsafe extern "system" fn currentPositionTimecode<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentPositionTimecode(::core::mem::transmute_copy(&bstrtimecode)).into()
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentPositionTimecode(::core::mem::transmute_copy(&bstrtimecode)).into()
        }
        Self {
            base: IWMPControls2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPConvert_Impl: Sized {
    fn ConvertFile(&mut self, bstrinputfile: super::super::Foundation::BSTR, bstrdestinationfolder: super::super::Foundation::BSTR, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetErrorURL(&mut self, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPConvert_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvert_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPConvert_Vtbl {
        unsafe extern "system" fn ConvertFile<Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertFile(::core::mem::transmute_copy(&bstrinputfile), ::core::mem::transmute_copy(&bstrdestinationfolder), ::core::mem::transmute_copy(&pbstroutputfile)).into()
        }
        unsafe extern "system" fn GetErrorURL<Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetErrorURL(::core::mem::transmute_copy(&pbstrurl)).into()
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
pub trait IWMPCore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn close(&mut self) -> ::windows::core::Result<()>;
    fn URL(&mut self, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetURL(&mut self, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn openState(&mut self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()>;
    fn playState(&mut self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()>;
    fn controls(&mut self) -> ::windows::core::Result<IWMPControls>;
    fn settings(&mut self) -> ::windows::core::Result<IWMPSettings>;
    fn currentMedia(&mut self) -> ::windows::core::Result<IWMPMedia>;
    fn SetcurrentMedia(&mut self, pmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn mediaCollection(&mut self) -> ::windows::core::Result<IWMPMediaCollection>;
    fn playlistCollection(&mut self) -> ::windows::core::Result<IWMPPlaylistCollection>;
    fn versionInfo(&mut self, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn launchURL(&mut self, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn network(&mut self) -> ::windows::core::Result<IWMPNetwork>;
    fn currentPlaylist(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetcurrentPlaylist(&mut self, ppl: ::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cdromCollection(&mut self) -> ::windows::core::Result<IWMPCdromCollection>;
    fn closedCaption(&mut self) -> ::windows::core::Result<IWMPClosedCaption>;
    fn isOnline(&mut self, pfonline: *mut i16) -> ::windows::core::Result<()>;
    fn error(&mut self) -> ::windows::core::Result<IWMPError>;
    fn status(&mut self, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCore_Vtbl {
        unsafe extern "system" fn close<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).close().into()
        }
        unsafe extern "system" fn URL<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn SetURL<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetURL(::core::mem::transmute_copy(&bstrurl)).into()
        }
        unsafe extern "system" fn openState<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).openState(::core::mem::transmute_copy(&pwmpos)).into()
        }
        unsafe extern "system" fn playState<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).playState(::core::mem::transmute_copy(&pwmpps)).into()
        }
        unsafe extern "system" fn controls<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).controls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn settings<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).settings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMedia<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMedia<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentMedia(::core::mem::transmute(&pmedia)).into()
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediacollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlistCollection<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playlistCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn versionInfo<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).versionInfo(::core::mem::transmute_copy(&pbstrversioninfo)).into()
        }
        unsafe extern "system" fn launchURL<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).launchURL(::core::mem::transmute_copy(&bstrurl)).into()
        }
        unsafe extern "system" fn network<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqni: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).network() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqni = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPlaylist<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).currentPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    *pppl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPlaylist<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcurrentPlaylist(::core::mem::transmute(&ppl)).into()
        }
        unsafe extern "system" fn cdromCollection<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).cdromCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcdromcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn closedCaption<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).closedCaption() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclosedcaption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOnline<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfonline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isOnline(::core::mem::transmute_copy(&pfonline)).into()
        }
        unsafe extern "system" fn error<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).error() {
                ::core::result::Result::Ok(ok__) => {
                    *pperror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).status(::core::mem::transmute_copy(&pbstrstatus)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPCore2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl {
    fn dvd(&mut self) -> ::windows::core::Result<IWMPDVD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCore2_Vtbl {
        unsafe extern "system" fn dvd<Impl: IWMPCore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).dvd() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdvd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMPCore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), dvd: dvd::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl + IWMPCore2_Impl {
    fn newPlaylist(&mut self, bstrname: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn newMedia(&mut self, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPMedia>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPCore3_Vtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newPlaylist(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn newMedia<Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newMedia(::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMPCore2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            newPlaylist: newPlaylist::<Impl, IMPL_OFFSET>,
            newMedia: newMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDVD_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn domain(&mut self, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn topMenu(&mut self) -> ::windows::core::Result<()>;
    fn titleMenu(&mut self) -> ::windows::core::Result<()>;
    fn back(&mut self) -> ::windows::core::Result<()>;
    fn resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDVD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDVD_Vtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn domain<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).domain(::core::mem::transmute_copy(&strdomain)).into()
        }
        unsafe extern "system" fn topMenu<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).topMenu().into()
        }
        unsafe extern "system" fn titleMenu<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).titleMenu().into()
        }
        unsafe extern "system" fn back<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).back().into()
        }
        unsafe extern "system" fn resume<Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resume().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPDownloadCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn id(&mut self, plid: *mut i32) -> ::windows::core::Result<()>;
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, litem: i32) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn startDownload(&mut self, bstrsourceurl: super::super::Foundation::BSTR, bstrtype: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn removeItem(&mut self, litem: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadCollection_Vtbl {
        unsafe extern "system" fn id<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).id(::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn count<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&litem)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDownload<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).startDownload(::core::mem::transmute_copy(&bstrsourceurl), ::core::mem::transmute_copy(&bstrtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeItem(::core::mem::transmute_copy(&litem)).into()
        }
        unsafe extern "system" fn Clear<Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPDownloadItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn sourceURL(&mut self, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn size(&mut self, plsize: *mut i32) -> ::windows::core::Result<()>;
    fn r#type(&mut self, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn progress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn downloadState(&mut self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::Result<()>;
    fn pause(&mut self) -> ::windows::core::Result<()>;
    fn resume(&mut self) -> ::windows::core::Result<()>;
    fn cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadItem_Vtbl {
        unsafe extern "system" fn sourceURL<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).sourceURL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn size<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).size(::core::mem::transmute_copy(&plsize)).into()
        }
        unsafe extern "system" fn r#type<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).r#type(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn progress<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn downloadState<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).downloadState(::core::mem::transmute_copy(&pwmpsdls)).into()
        }
        unsafe extern "system" fn pause<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pause().into()
        }
        unsafe extern "system" fn resume<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).resume().into()
        }
        unsafe extern "system" fn cancel<Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).cancel().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPDownloadItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPDownloadItem_Impl {
    fn getItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadItem2_Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPDownloadItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base: IWMPDownloadItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), getItemInfo: getItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn getDownloadCollection(&mut self, lcollectionid: i32) -> ::windows::core::Result<IWMPDownloadCollection>;
    fn createDownloadCollection(&mut self) -> ::windows::core::Result<IWMPDownloadCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPDownloadManager_Vtbl {
        unsafe extern "system" fn getDownloadCollection<Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDownloadCollection(::core::mem::transmute_copy(&lcollectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDownloadCollection<Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createDownloadCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getDownloadCollection: getDownloadCollection::<Impl, IMPL_OFFSET>,
            createDownloadCollection: createDownloadCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IWMPEffects_Impl: Sized {
    fn Render(&mut self, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn MediaInfo(&mut self, lchannelcount: i32, lsamplerate: i32, bstrtitle: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCapabilities(&mut self, pdwcapabilities: *mut u32) -> ::windows::core::Result<()>;
    fn GetTitle(&mut self, bstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPresetTitle(&mut self, npreset: i32, bstrpresettitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPresetCount(&mut self, pnpresetcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetCurrentPreset(&mut self, npreset: i32) -> ::windows::core::Result<()>;
    fn GetCurrentPreset(&mut self, pnpreset: *mut i32) -> ::windows::core::Result<()>;
    fn DisplayPropertyPage(&mut self, hwndowner: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GoFullscreen(&mut self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RenderFullScreen(&mut self, plevels: *mut TimedLevel) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IWMPEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEffects_Vtbl {
        unsafe extern "system" fn Render<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&plevels), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn MediaInfo<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaInfo(::core::mem::transmute_copy(&lchannelcount), ::core::mem::transmute_copy(&lsamplerate), ::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilities)).into()
        }
        unsafe extern "system" fn GetTitle<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetPresetTitle<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresetTitle(::core::mem::transmute_copy(&npreset), ::core::mem::transmute_copy(&bstrpresettitle)).into()
        }
        unsafe extern "system" fn GetPresetCount<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresetCount(::core::mem::transmute_copy(&pnpresetcount)).into()
        }
        unsafe extern "system" fn SetCurrentPreset<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentPreset(::core::mem::transmute_copy(&npreset)).into()
        }
        unsafe extern "system" fn GetCurrentPreset<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPreset(::core::mem::transmute_copy(&pnpreset)).into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayPropertyPage(::core::mem::transmute_copy(&hwndowner)).into()
        }
        unsafe extern "system" fn GoFullscreen<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GoFullscreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn RenderFullScreen<Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenderFullScreen(::core::mem::transmute_copy(&plevels)).into()
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
pub trait IWMPEffects2_Impl: Sized + IWMPEffects_Impl {
    fn SetCore(&mut self, pplayer: ::core::option::Option<IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn NotifyNewMedia(&mut self, pmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn OnWindowMessage(&mut self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn RenderWindowed(&mut self, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IWMPEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEffects2_Vtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCore(::core::mem::transmute(&pplayer)).into()
        }
        unsafe extern "system" fn Create<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Destroy<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn NotifyNewMedia<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyNewMedia(::core::mem::transmute(&pmedia)).into()
        }
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresultparam)).into()
        }
        unsafe extern "system" fn RenderWindowed<Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenderWindowed(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&frequiredrender)).into()
        }
        Self {
            base: IWMPEffects_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPError_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn clearErrorQueue(&mut self) -> ::windows::core::Result<()>;
    fn errorCount(&mut self, plnumerrors: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, dwindex: i32) -> ::windows::core::Result<IWMPErrorItem>;
    fn webHelp(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPError_Vtbl {
        unsafe extern "system" fn clearErrorQueue<Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).clearErrorQueue().into()
        }
        unsafe extern "system" fn errorCount<Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).errorCount(::core::mem::transmute_copy(&plnumerrors)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pperroritem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn webHelp<Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).webHelp().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPErrorItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn errorCode(&mut self, phr: *mut i32) -> ::windows::core::Result<()>;
    fn errorDescription(&mut self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn errorContext(&mut self, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn remedy(&mut self, plremedy: *mut i32) -> ::windows::core::Result<()>;
    fn customUrl(&mut self, pbstrcustomurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPErrorItem_Vtbl {
        unsafe extern "system" fn errorCode<Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).errorCode(::core::mem::transmute_copy(&phr)).into()
        }
        unsafe extern "system" fn errorDescription<Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).errorDescription(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn errorContext<Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).errorContext(::core::mem::transmute_copy(&pvarcontext)).into()
        }
        unsafe extern "system" fn remedy<Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).remedy(::core::mem::transmute_copy(&plremedy)).into()
        }
        unsafe extern "system" fn customUrl<Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).customUrl(::core::mem::transmute_copy(&pbstrcustomurl)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPErrorItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPErrorItem_Impl {
    fn condition(&mut self, plcondition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPErrorItem2_Vtbl {
        unsafe extern "system" fn condition<Impl: IWMPErrorItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).condition(::core::mem::transmute_copy(&plcondition)).into()
        }
        Self { base: IWMPErrorItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), condition: condition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents_Impl: Sized {
    fn OpenStateChange(&mut self, newstate: i32);
    fn PlayStateChange(&mut self, newstate: i32);
    fn AudioLanguageChange(&mut self, langid: i32);
    fn StatusChange(&mut self);
    fn ScriptCommand(&mut self, sctype: super::super::Foundation::BSTR, param: super::super::Foundation::BSTR);
    fn NewStream(&mut self);
    fn Disconnect(&mut self, result: i32);
    fn Buffering(&mut self, start: i16);
    fn Error(&mut self);
    fn Warning(&mut self, warningtype: i32, param: i32, description: super::super::Foundation::BSTR);
    fn EndOfStream(&mut self, result: i32);
    fn PositionChange(&mut self, oldposition: f64, newposition: f64);
    fn MarkerHit(&mut self, markernum: i32);
    fn DurationUnitChange(&mut self, newdurationunit: i32);
    fn CdromMediaChange(&mut self, cdromnum: i32);
    fn PlaylistChange(&mut self, playlist: ::core::option::Option<super::super::System::Com::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(&mut self, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(&mut self, bstritemname: super::super::Foundation::BSTR);
    fn MediaChange(&mut self, item: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn CurrentMediaItemAvailable(&mut self, bstritemname: super::super::Foundation::BSTR);
    fn CurrentItemChange(&mut self, pdispmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn MediaCollectionChange(&mut self);
    fn MediaCollectionAttributeStringAdded(&mut self, bstrattribname: super::super::Foundation::BSTR, bstrattribval: super::super::Foundation::BSTR);
    fn MediaCollectionAttributeStringRemoved(&mut self, bstrattribname: super::super::Foundation::BSTR, bstrattribval: super::super::Foundation::BSTR);
    fn MediaCollectionAttributeStringChanged(&mut self, bstrattribname: super::super::Foundation::BSTR, bstroldattribval: super::super::Foundation::BSTR, bstrnewattribval: super::super::Foundation::BSTR);
    fn PlaylistCollectionChange(&mut self);
    fn PlaylistCollectionPlaylistAdded(&mut self, bstrplaylistname: super::super::Foundation::BSTR);
    fn PlaylistCollectionPlaylistRemoved(&mut self, bstrplaylistname: super::super::Foundation::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(&mut self, bstrplaylistname: super::super::Foundation::BSTR, varfisdeleted: i16);
    fn ModeChange(&mut self, modename: super::super::Foundation::BSTR, newvalue: i16);
    fn MediaError(&mut self, pmediaobject: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn OpenPlaylistSwitch(&mut self, pitem: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn DomainChange(&mut self, strdomain: super::super::Foundation::BSTR);
    fn SwitchedToPlayerApplication(&mut self);
    fn SwitchedToControl(&mut self);
    fn PlayerDockedStateChange(&mut self);
    fn PlayerReconnect(&mut self);
    fn Click(&mut self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn DoubleClick(&mut self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn KeyDown(&mut self, nkeycode: i16, nshiftstate: i16);
    fn KeyPress(&mut self, nkeyascii: i16);
    fn KeyUp(&mut self, nkeycode: i16, nshiftstate: i16);
    fn MouseDown(&mut self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseMove(&mut self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseUp(&mut self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents_Vtbl {
        unsafe extern "system" fn OpenStateChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn PlayStateChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn AudioLanguageChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AudioLanguageChange(::core::mem::transmute_copy(&langid))
        }
        unsafe extern "system" fn StatusChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StatusChange()
        }
        unsafe extern "system" fn ScriptCommand<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sctype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, param: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScriptCommand(::core::mem::transmute_copy(&sctype), ::core::mem::transmute_copy(&param))
        }
        unsafe extern "system" fn NewStream<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewStream()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn Buffering<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Buffering(::core::mem::transmute_copy(&start))
        }
        unsafe extern "system" fn Error<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error()
        }
        unsafe extern "system" fn Warning<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Warning(::core::mem::transmute_copy(&warningtype), ::core::mem::transmute_copy(&param), ::core::mem::transmute_copy(&description))
        }
        unsafe extern "system" fn EndOfStream<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOfStream(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn PositionChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PositionChange(::core::mem::transmute_copy(&oldposition), ::core::mem::transmute_copy(&newposition))
        }
        unsafe extern "system" fn MarkerHit<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarkerHit(::core::mem::transmute_copy(&markernum))
        }
        unsafe extern "system" fn DurationUnitChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DurationUnitChange(::core::mem::transmute_copy(&newdurationunit))
        }
        unsafe extern "system" fn CdromMediaChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromMediaChange(::core::mem::transmute_copy(&cdromnum))
        }
        unsafe extern "system" fn PlaylistChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlist: ::windows::core::RawPtr, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistChange(::core::mem::transmute(&playlist), ::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentPlaylistChange(::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentPlaylistItemAvailable(::core::mem::transmute_copy(&bstritemname))
        }
        unsafe extern "system" fn MediaChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaChange(::core::mem::transmute(&item))
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentMediaItemAvailable(::core::mem::transmute_copy(&bstritemname))
        }
        unsafe extern "system" fn CurrentItemChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CurrentItemChange(::core::mem::transmute(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionChange()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionAttributeStringAdded(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionAttributeStringRemoved(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroldattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionAttributeStringChanged(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstroldattribval), ::core::mem::transmute_copy(&bstrnewattribval))
        }
        unsafe extern "system" fn PlaylistCollectionChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionChange()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistAdded(::core::mem::transmute_copy(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistRemoved(::core::mem::transmute_copy(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfisdeleted: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlaylistCollectionPlaylistSetAsDeleted(::core::mem::transmute_copy(&bstrplaylistname), ::core::mem::transmute_copy(&varfisdeleted))
        }
        unsafe extern "system" fn ModeChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newvalue: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModeChange(::core::mem::transmute_copy(&modename), ::core::mem::transmute_copy(&newvalue))
        }
        unsafe extern "system" fn MediaError<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediaobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaError(::core::mem::transmute(&pmediaobject))
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenPlaylistSwitch(::core::mem::transmute(&pitem))
        }
        unsafe extern "system" fn DomainChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DomainChange(::core::mem::transmute_copy(&strdomain))
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchedToPlayerApplication()
        }
        unsafe extern "system" fn SwitchedToControl<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchedToControl()
        }
        unsafe extern "system" fn PlayerDockedStateChange<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayerDockedStateChange()
        }
        unsafe extern "system" fn PlayerReconnect<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayerReconnect()
        }
        unsafe extern "system" fn Click<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Click(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn DoubleClick<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoubleClick(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn KeyDown<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyDown(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn KeyPress<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyPress(::core::mem::transmute_copy(&nkeyascii))
        }
        unsafe extern "system" fn KeyUp<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KeyUp(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn MouseDown<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseDown(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseMove<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseMove(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseUp<Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseUp(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
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
pub trait IWMPEvents2_Impl: Sized + IWMPEvents_Impl {
    fn DeviceConnect(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>);
    fn DeviceDisconnect(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>);
    fn DeviceStatusChange(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, pmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn CreatePartnershipComplete(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, hrresult: ::windows::core::HRESULT);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents2_Vtbl {
        unsafe extern "system" fn DeviceConnect<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceConnect(::core::mem::transmute(&pdevice))
        }
        unsafe extern "system" fn DeviceDisconnect<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceDisconnect(::core::mem::transmute(&pdevice))
        }
        unsafe extern "system" fn DeviceStatusChange<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstatus: WMPDeviceStatus) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceStatusChange(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&newstatus))
        }
        unsafe extern "system" fn DeviceSyncStateChange<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstate: WMPSyncState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSyncStateChange(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn DeviceSyncError<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSyncError(::core::mem::transmute(&pdevice), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CreatePartnershipComplete<Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePartnershipComplete(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hrresult))
        }
        Self {
            base: IWMPEvents_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPEvents3_Impl: Sized + IWMPEvents_Impl + IWMPEvents2_Impl {
    fn CdromRipStateChange(&mut self, pcdromrip: ::core::option::Option<IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(&mut self, pcdromrip: ::core::option::Option<IWMPCdromRip>, pmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn CdromBurnStateChange(&mut self, pcdromburn: ::core::option::Option<IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(&mut self, pcdromburn: ::core::option::Option<IWMPCdromBurn>, pmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn CdromBurnError(&mut self, pcdromburn: ::core::option::Option<IWMPCdromBurn>, hrerror: ::windows::core::HRESULT);
    fn LibraryConnect(&mut self, plibrary: ::core::option::Option<IWMPLibrary>);
    fn LibraryDisconnect(&mut self, plibrary: ::core::option::Option<IWMPLibrary>);
    fn FolderScanStateChange(&mut self, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(&mut self, pdispstringcollection: ::core::option::Option<super::super::System::Com::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(&mut self, pdispmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
    fn MediaCollectionMediaRemoved(&mut self, pdispmedia: ::core::option::Option<super::super::System::Com::IDispatch>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents3_Vtbl {
        unsafe extern "system" fn CdromRipStateChange<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, wmprs: WMPRipState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromRipStateChange(::core::mem::transmute(&pcdromrip), ::core::mem::transmute_copy(&wmprs))
        }
        unsafe extern "system" fn CdromRipMediaError<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromRipMediaError(::core::mem::transmute(&pcdromrip), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CdromBurnStateChange<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, wmpbs: WMPBurnState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnStateChange(::core::mem::transmute(&pcdromburn), ::core::mem::transmute_copy(&wmpbs))
        }
        unsafe extern "system" fn CdromBurnMediaError<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnMediaError(::core::mem::transmute(&pcdromburn), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CdromBurnError<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CdromBurnError(::core::mem::transmute(&pcdromburn), ::core::mem::transmute_copy(&hrerror))
        }
        unsafe extern "system" fn LibraryConnect<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LibraryConnect(::core::mem::transmute(&plibrary))
        }
        unsafe extern "system" fn LibraryDisconnect<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LibraryDisconnect(::core::mem::transmute(&plibrary))
        }
        unsafe extern "system" fn FolderScanStateChange<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderScanStateChange(::core::mem::transmute_copy(&wmpfss))
        }
        unsafe extern "system" fn StringCollectionChange<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispstringcollection: ::windows::core::RawPtr, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StringCollectionChange(::core::mem::transmute(&pdispstringcollection), ::core::mem::transmute_copy(&change), ::core::mem::transmute_copy(&lcollectionindex))
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionMediaAdded(::core::mem::transmute(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MediaCollectionMediaRemoved(::core::mem::transmute(&pdispmedia))
        }
        Self {
            base: IWMPEvents2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPEvents4_Impl: Sized + IWMPEvents_Impl + IWMPEvents2_Impl + IWMPEvents3_Impl {
    fn DeviceEstimation(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPEvents4_Vtbl {
        unsafe extern "system" fn DeviceEstimation<Impl: IWMPEvents4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceEstimation(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&qwestimatedusedspace), ::core::mem::transmute_copy(&qwestimatedspace))
        }
        Self { base: IWMPEvents3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DeviceEstimation: DeviceEstimation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPFolderMonitorServices_Impl: Sized {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn add(&mut self, bstrfolder: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn remove(&mut self, lindex: i32) -> ::windows::core::Result<()>;
    fn scanState(&mut self, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::Result<()>;
    fn currentFolder(&mut self, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn scannedFilesCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn addedFilesCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn updateProgress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startScan(&mut self) -> ::windows::core::Result<()>;
    fn stopScan(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPFolderMonitorServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPFolderMonitorServices_Vtbl {
        unsafe extern "system" fn count<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn add<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).add(::core::mem::transmute_copy(&bstrfolder)).into()
        }
        unsafe extern "system" fn remove<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn scanState<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).scanState(::core::mem::transmute_copy(&pwmpfss)).into()
        }
        unsafe extern "system" fn currentFolder<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).currentFolder(::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn scannedFilesCount<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).scannedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn addedFilesCount<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn updateProgress<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).updateProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startScan<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startScan().into()
        }
        unsafe extern "system" fn stopScan<Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stopScan().into()
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
pub trait IWMPGraphCreation_Impl: Sized {
    fn GraphCreationPreRender(&mut self, pfiltergraph: ::core::option::Option<::windows::core::IUnknown>, preserved: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GraphCreationPostRender(&mut self, pfiltergraph: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetGraphCreationFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMPGraphCreation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPGraphCreation_Vtbl {
        unsafe extern "system" fn GraphCreationPreRender<Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GraphCreationPreRender(::core::mem::transmute(&pfiltergraph), ::core::mem::transmute(&preserved)).into()
        }
        unsafe extern "system" fn GraphCreationPostRender<Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GraphCreationPostRender(::core::mem::transmute(&pfiltergraph)).into()
        }
        unsafe extern "system" fn GetGraphCreationFlags<Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGraphCreationFlags(::core::mem::transmute_copy(&pdwflags)).into()
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
pub trait IWMPLibrary_Impl: Sized {
    fn name(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn r#type(&mut self, pwmplt: *mut WMPLibraryType) -> ::windows::core::Result<()>;
    fn mediaCollection(&mut self) -> ::windows::core::Result<IWMPMediaCollection>;
    fn isIdentical(&mut self, piwmplibrary: ::core::option::Option<IWMPLibrary>, pvbool: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibrary_Vtbl {
        unsafe extern "system" fn name<Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn r#type<Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).r#type(::core::mem::transmute_copy(&pwmplt)).into()
        }
        unsafe extern "system" fn mediaCollection<Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmediacollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmplibrary: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmplibrary), ::core::mem::transmute_copy(&pvbool)).into()
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
pub trait IWMPLibrary2_Impl: Sized + IWMPLibrary_Impl {
    fn getItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibrary2_Vtbl {
        unsafe extern "system" fn getItemInfo<Impl: IWMPLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base: IWMPLibrary_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), getItemInfo: getItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPLibraryServices_Impl: Sized {
    fn getCountByType(&mut self, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getLibraryByType(&mut self, wmplt: WMPLibraryType, lindex: i32) -> ::windows::core::Result<IWMPLibrary>;
}
impl IWMPLibraryServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibraryServices_Vtbl {
        unsafe extern "system" fn getCountByType<Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getCountByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getLibraryByType<Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getLibraryByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmplibrary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IWMPLibrarySharingServices_Impl: Sized {
    fn isLibraryShared(&mut self, pvbshared: *mut i16) -> ::windows::core::Result<()>;
    fn isLibrarySharingEnabled(&mut self, pvbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn showLibrarySharing(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPLibrarySharingServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPLibrarySharingServices_Vtbl {
        unsafe extern "system" fn isLibraryShared<Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isLibraryShared(::core::mem::transmute_copy(&pvbshared)).into()
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isLibrarySharingEnabled(::core::mem::transmute_copy(&pvbenabled)).into()
        }
        unsafe extern "system" fn showLibrarySharing<Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).showLibrarySharing().into()
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
pub trait IWMPMedia_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isIdentical(&mut self, piwmpmedia: ::core::option::Option<IWMPMedia>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn sourceURL(&mut self, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn name(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn imageSourceWidth(&mut self, pwidth: *mut i32) -> ::windows::core::Result<()>;
    fn imageSourceHeight(&mut self, pheight: *mut i32) -> ::windows::core::Result<()>;
    fn markerCount(&mut self, pmarkercount: *mut i32) -> ::windows::core::Result<()>;
    fn getMarkerTime(&mut self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()>;
    fn getMarkerName(&mut self, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn duration(&mut self, pduration: *mut f64) -> ::windows::core::Result<()>;
    fn durationString(&mut self, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getAttributeName(&mut self, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfoByAtom(&mut self, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn isMemberOf(&mut self, pplaylist: ::core::option::Option<IWMPPlaylist>, pvarfismemberof: *mut i16) -> ::windows::core::Result<()>;
    fn isReadOnlyItem(&mut self, bstritemname: super::super::Foundation::BSTR, pvarfisreadonly: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMedia_Vtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpmedia), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn sourceURL<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).sourceURL(::core::mem::transmute_copy(&pbstrsourceurl)).into()
        }
        unsafe extern "system" fn name<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setname(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn imageSourceWidth<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).imageSourceWidth(::core::mem::transmute_copy(&pwidth)).into()
        }
        unsafe extern "system" fn imageSourceHeight<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).imageSourceHeight(::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn markerCount<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).markerCount(::core::mem::transmute_copy(&pmarkercount)).into()
        }
        unsafe extern "system" fn getMarkerTime<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getMarkerTime(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pmarkertime)).into()
        }
        unsafe extern "system" fn getMarkerName<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getMarkerName(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pbstrmarkername)).into()
        }
        unsafe extern "system" fn duration<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn durationString<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).durationString(::core::mem::transmute_copy(&pbstrduration)).into()
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAttributeName<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAttributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstritemname)).into()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn getItemInfoByAtom<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfoByAtom(::core::mem::transmute_copy(&latom), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn isMemberOf<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr, pvarfismemberof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isMemberOf(::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pvarfismemberof)).into()
        }
        unsafe extern "system" fn isReadOnlyItem<Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfisreadonly: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isReadOnlyItem(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pvarfisreadonly)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPMedia2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMedia_Impl {
    fn error(&mut self) -> ::windows::core::Result<IWMPErrorItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMedia2_Vtbl {
        unsafe extern "system" fn error<Impl: IWMPMedia2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).error() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmperroritem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMPMedia_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), error: error::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMedia_Impl + IWMPMedia2_Impl {
    fn getAttributeCountByType(&mut self, bstrtype: super::super::Foundation::BSTR, bstrlanguage: super::super::Foundation::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&mut self, bstrtype: super::super::Foundation::BSTR, bstrlanguage: super::super::Foundation::BSTR, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMedia3_Vtbl {
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAttributeCountByType(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfoByType(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base: IWMPMedia2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Impl, IMPL_OFFSET>,
            getItemInfoByType: getItemInfoByType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn add(&mut self, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPMedia>;
    fn getAll(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByGenre(&mut self, bstrgenre: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAuthor(&mut self, bstrauthor: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAlbum(&mut self, bstralbum: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAttribute(&mut self, bstrattribute: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn remove(&mut self, pitem: ::core::option::Option<IWMPMedia>, varfdeletefile: i16) -> ::windows::core::Result<()>;
    fn getAttributeStringCollection(&mut self, bstrattribute: super::super::Foundation::BSTR, bstrmediatype: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPStringCollection>;
    fn getMediaAtom(&mut self, bstritemname: super::super::Foundation::BSTR, platom: *mut i32) -> ::windows::core::Result<()>;
    fn setDeleted(&mut self, pitem: ::core::option::Option<IWMPMedia>, varfisdeleted: i16) -> ::windows::core::Result<()>;
    fn isDeleted(&mut self, pitem: ::core::option::Option<IWMPMedia>, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaCollection_Vtbl {
        unsafe extern "system" fn add<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).add(::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByGenre<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgenre: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByGenre(::core::mem::transmute_copy(&bstrgenre)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAuthor<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAuthor(::core::mem::transmute_copy(&bstrauthor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAlbum<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstralbum: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAlbum(::core::mem::transmute_copy(&bstralbum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttribute<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAttribute(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfdeletefile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).remove(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfdeletefile)).into()
        }
        unsafe extern "system" fn getAttributeStringCollection<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAttributeStringCollection(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstringcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMediaAtom<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getMediaAtom(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&platom)).into()
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPMediaCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMediaCollection_Impl {
    fn createQuery(&mut self) -> ::windows::core::Result<IWMPQuery>;
    fn getPlaylistByQuery(&mut self, pquery: ::core::option::Option<IWMPQuery>, bstrmediatype: super::super::Foundation::BSTR, bstrsortattribute: super::super::Foundation::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(&mut self, bstrattribute: super::super::Foundation::BSTR, pquery: ::core::option::Option<IWMPQuery>, bstrmediatype: super::super::Foundation::BSTR, bstrsortattribute: super::super::Foundation::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(&mut self, bstrattribute: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR, bstrmediatype: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaCollection2_Vtbl {
        unsafe extern "system" fn createQuery<Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).createQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getPlaylistByQuery(::core::mem::transmute(&pquery), ::core::mem::transmute_copy(&bstrmediatype), ::core::mem::transmute_copy(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getStringCollectionByQuery(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute(&pquery), ::core::mem::transmute_copy(&bstrmediatype), ::core::mem::transmute_copy(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstringcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByAttributeAndMediaType(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrvalue), ::core::mem::transmute_copy(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMPMediaCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPMediaPluginRegistrar_Impl: Sized {
    fn WMPRegisterPlayerPlugin(&mut self, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn WMPUnRegisterPlayerPlugin(&mut self, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPMediaPluginRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMediaPluginRegistrar_Vtbl {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMPRegisterPlayerPlugin(::core::mem::transmute_copy(&pwszfriendlyname), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pwszuninstallstring), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute_copy(&guidplugintype), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&cmediatypes), ::core::mem::transmute_copy(&pmediatypes)).into()
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMPUnRegisterPlayerPlugin(::core::mem::transmute_copy(&guidplugintype), ::core::mem::transmute_copy(&clsid)).into()
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
pub trait IWMPMetadataPicture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn mimeType(&mut self, pbstrmimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn pictureType(&mut self, pbstrpicturetype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn description(&mut self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn URL(&mut self, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataPicture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMetadataPicture_Vtbl {
        unsafe extern "system" fn mimeType<Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).mimeType(::core::mem::transmute_copy(&pbstrmimetype)).into()
        }
        unsafe extern "system" fn pictureType<Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).pictureType(::core::mem::transmute_copy(&pbstrpicturetype)).into()
        }
        unsafe extern "system" fn description<Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn URL<Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPMetadataText_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn description(&mut self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn text(&mut self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataText_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPMetadataText_Vtbl {
        unsafe extern "system" fn description<Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn text<Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).text(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            description: description::<Impl, IMPL_OFFSET>,
            text: text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn bandWidth(&mut self, plbandwidth: *mut i32) -> ::windows::core::Result<()>;
    fn recoveredPackets(&mut self, plrecoveredpackets: *mut i32) -> ::windows::core::Result<()>;
    fn sourceProtocol(&mut self, pbstrsourceprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn receivedPackets(&mut self, plreceivedpackets: *mut i32) -> ::windows::core::Result<()>;
    fn lostPackets(&mut self, pllostpackets: *mut i32) -> ::windows::core::Result<()>;
    fn receptionQuality(&mut self, plreceptionquality: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingCount(&mut self, plbufferingcount: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingProgress(&mut self, plbufferingprogress: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingTime(&mut self, plbufferingtime: *mut i32) -> ::windows::core::Result<()>;
    fn SetbufferingTime(&mut self, lbufferingtime: i32) -> ::windows::core::Result<()>;
    fn frameRate(&mut self, plframerate: *mut i32) -> ::windows::core::Result<()>;
    fn maxBitRate(&mut self, plbitrate: *mut i32) -> ::windows::core::Result<()>;
    fn bitRate(&mut self, plbitrate: *mut i32) -> ::windows::core::Result<()>;
    fn getProxySettings(&mut self, bstrprotocol: super::super::Foundation::BSTR, plproxysetting: *mut i32) -> ::windows::core::Result<()>;
    fn setProxySettings(&mut self, bstrprotocol: super::super::Foundation::BSTR, lproxysetting: i32) -> ::windows::core::Result<()>;
    fn getProxyName(&mut self, bstrprotocol: super::super::Foundation::BSTR, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setProxyName(&mut self, bstrprotocol: super::super::Foundation::BSTR, bstrproxyname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getProxyPort(&mut self, bstrprotocol: super::super::Foundation::BSTR, lproxyport: *mut i32) -> ::windows::core::Result<()>;
    fn setProxyPort(&mut self, bstrprotocol: super::super::Foundation::BSTR, lproxyport: i32) -> ::windows::core::Result<()>;
    fn getProxyExceptionList(&mut self, bstrprotocol: super::super::Foundation::BSTR, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setProxyExceptionList(&mut self, bstrprotocol: super::super::Foundation::BSTR, pbstrexceptionlist: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getProxyBypassForLocal(&mut self, bstrprotocol: super::super::Foundation::BSTR, pfbypassforlocal: *mut i16) -> ::windows::core::Result<()>;
    fn setProxyBypassForLocal(&mut self, bstrprotocol: super::super::Foundation::BSTR, fbypassforlocal: i16) -> ::windows::core::Result<()>;
    fn maxBandwidth(&mut self, lmaxbandwidth: *mut i32) -> ::windows::core::Result<()>;
    fn SetmaxBandwidth(&mut self, lmaxbandwidth: i32) -> ::windows::core::Result<()>;
    fn downloadProgress(&mut self, pldownloadprogress: *mut i32) -> ::windows::core::Result<()>;
    fn encodedFrameRate(&mut self, plframerate: *mut i32) -> ::windows::core::Result<()>;
    fn framesSkipped(&mut self, plframes: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNetwork_Vtbl {
        unsafe extern "system" fn bandWidth<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).bandWidth(::core::mem::transmute_copy(&plbandwidth)).into()
        }
        unsafe extern "system" fn recoveredPackets<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).recoveredPackets(::core::mem::transmute_copy(&plrecoveredpackets)).into()
        }
        unsafe extern "system" fn sourceProtocol<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).sourceProtocol(::core::mem::transmute_copy(&pbstrsourceprotocol)).into()
        }
        unsafe extern "system" fn receivedPackets<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).receivedPackets(::core::mem::transmute_copy(&plreceivedpackets)).into()
        }
        unsafe extern "system" fn lostPackets<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).lostPackets(::core::mem::transmute_copy(&pllostpackets)).into()
        }
        unsafe extern "system" fn receptionQuality<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).receptionQuality(::core::mem::transmute_copy(&plreceptionquality)).into()
        }
        unsafe extern "system" fn bufferingCount<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).bufferingCount(::core::mem::transmute_copy(&plbufferingcount)).into()
        }
        unsafe extern "system" fn bufferingProgress<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).bufferingProgress(::core::mem::transmute_copy(&plbufferingprogress)).into()
        }
        unsafe extern "system" fn bufferingTime<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).bufferingTime(::core::mem::transmute_copy(&plbufferingtime)).into()
        }
        unsafe extern "system" fn SetbufferingTime<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetbufferingTime(::core::mem::transmute_copy(&lbufferingtime)).into()
        }
        unsafe extern "system" fn frameRate<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).frameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn maxBitRate<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).maxBitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn bitRate<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).bitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn getProxySettings<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProxySettings(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&plproxysetting)).into()
        }
        unsafe extern "system" fn setProxySettings<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxySettings(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxysetting)).into()
        }
        unsafe extern "system" fn getProxyName<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProxyName(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrproxyname)).into()
        }
        unsafe extern "system" fn setProxyName<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxyName(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrproxyname)).into()
        }
        unsafe extern "system" fn getProxyPort<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProxyPort(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn setProxyPort<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxyPort(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn getProxyExceptionList<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProxyExceptionList(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn setProxyExceptionList<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxyExceptionList(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn getProxyBypassForLocal<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfbypassforlocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getProxyBypassForLocal(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pfbypassforlocal)).into()
        }
        unsafe extern "system" fn setProxyBypassForLocal<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fbypassforlocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setProxyBypassForLocal(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn maxBandwidth<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).maxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn SetmaxBandwidth<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetmaxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn downloadProgress<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).downloadProgress(::core::mem::transmute_copy(&pldownloadprogress)).into()
        }
        unsafe extern "system" fn encodedFrameRate<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).encodedFrameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn framesSkipped<Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).framesSkipped(::core::mem::transmute_copy(&plframes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPNodeRealEstate_Impl: Sized {
    fn GetDesiredSize(&mut self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn SetRects(&mut self, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetRects(&mut self, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetWindowless(&mut self, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetWindowless(&mut self, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetFullScreen(&mut self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFullScreen(&mut self, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeRealEstate_Vtbl {
        unsafe extern "system" fn GetDesiredSize<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesiredSize(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn SetRects<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn GetRects<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn SetWindowless<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWindowless(::core::mem::transmute_copy(&fwindowless)).into()
        }
        unsafe extern "system" fn GetWindowless<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWindowless(::core::mem::transmute_copy(&pfwindowless)).into()
        }
        unsafe extern "system" fn SetFullScreen<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullScreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn GetFullScreen<Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFullScreen(::core::mem::transmute_copy(&pffullscreen)).into()
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
pub trait IWMPNodeRealEstateHost_Impl: Sized {
    fn OnDesiredSizeChange(&mut self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn OnFullScreenTransition(&mut self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstateHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeRealEstateHost_Vtbl {
        unsafe extern "system" fn OnDesiredSizeChange<Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDesiredSizeChange(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn OnFullScreenTransition<Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFullScreenTransition(::core::mem::transmute_copy(&ffullscreen)).into()
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
pub trait IWMPNodeWindowed_Impl: Sized {
    fn SetOwnerWindow(&mut self, hwnd: isize) -> ::windows::core::Result<()>;
    fn GetOwnerWindow(&mut self, phwnd: *mut isize) -> ::windows::core::Result<()>;
}
impl IWMPNodeWindowed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowed_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowed_Vtbl {
        unsafe extern "system" fn SetOwnerWindow<Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwnerWindow(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetOwnerWindow<Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOwnerWindow(::core::mem::transmute_copy(&phwnd)).into()
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
pub trait IWMPNodeWindowedHost_Impl: Sized {
    fn OnWindowMessageFromRenderer(&mut self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowedHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowedHost_Vtbl {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Impl: IWMPNodeWindowedHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowMessageFromRenderer(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
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
pub trait IWMPNodeWindowless_Impl: Sized + IWMPWindowMessageSink_Impl {
    fn OnDraw(&mut self, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowless_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowless_Vtbl {
        unsafe extern "system" fn OnDraw<Impl: IWMPNodeWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDraw(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prcdraw)).into()
        }
        Self { base: IWMPWindowMessageSink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDraw: OnDraw::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowless as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessHost_Impl: Sized {
    fn InvalidateRect(&mut self, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowlessHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPNodeWindowlessHost_Vtbl {
        unsafe extern "system" fn InvalidateRect<Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateRect(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&ferase)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), InvalidateRect: InvalidateRect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowlessHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl {
    fn enabled(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn Setenabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn fullScreen(&mut self, pbfullscreen: *mut i16) -> ::windows::core::Result<()>;
    fn SetfullScreen(&mut self, bfullscreen: i16) -> ::windows::core::Result<()>;
    fn enableContextMenu(&mut self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&mut self, benablecontextmenu: i16) -> ::windows::core::Result<()>;
    fn SetuiMode(&mut self, bstrmode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer_Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        Self {
            base: IWMPCore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlayer2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl {
    fn enabled(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn Setenabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn fullScreen(&mut self, pbfullscreen: *mut i16) -> ::windows::core::Result<()>;
    fn SetfullScreen(&mut self, bfullscreen: i16) -> ::windows::core::Result<()>;
    fn enableContextMenu(&mut self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&mut self, benablecontextmenu: i16) -> ::windows::core::Result<()>;
    fn SetuiMode(&mut self, bstrmode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer2_Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IWMPCore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlayer3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl + IWMPCore2_Impl {
    fn enabled(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn Setenabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn fullScreen(&mut self, pbfullscreen: *mut i16) -> ::windows::core::Result<()>;
    fn SetfullScreen(&mut self, bfullscreen: i16) -> ::windows::core::Result<()>;
    fn enableContextMenu(&mut self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&mut self, benablecontextmenu: i16) -> ::windows::core::Result<()>;
    fn SetuiMode(&mut self, bstrmode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer3_Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IWMPCore2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlayer4_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl + IWMPCore2_Impl + IWMPCore3_Impl {
    fn enabled(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn Setenabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn fullScreen(&mut self, pbfullscreen: *mut i16) -> ::windows::core::Result<()>;
    fn SetfullScreen(&mut self, bfullscreen: i16) -> ::windows::core::Result<()>;
    fn enableContextMenu(&mut self, pbenablecontextmenu: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&mut self, benablecontextmenu: i16) -> ::windows::core::Result<()>;
    fn SetuiMode(&mut self, bstrmode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn isRemote(&mut self, pvarfisremote: *mut i16) -> ::windows::core::Result<()>;
    fn playerApplication(&mut self) -> ::windows::core::Result<IWMPPlayerApplication>;
    fn openPlayer(&mut self, bstrurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayer4_Vtbl {
        unsafe extern "system" fn enabled<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn isRemote<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isRemote(::core::mem::transmute_copy(&pvarfisremote)).into()
        }
        unsafe extern "system" fn playerApplication<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).playerApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpplayerapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openPlayer<Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).openPlayer(::core::mem::transmute_copy(&bstrurl)).into()
        }
        Self {
            base: IWMPCore3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlayerApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn switchToPlayerApplication(&mut self) -> ::windows::core::Result<()>;
    fn switchToControl(&mut self) -> ::windows::core::Result<()>;
    fn playerDocked(&mut self, pbplayerdocked: *mut i16) -> ::windows::core::Result<()>;
    fn hasDisplay(&mut self, pbhasdisplay: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayerApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerApplication_Vtbl {
        unsafe extern "system" fn switchToPlayerApplication<Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).switchToPlayerApplication().into()
        }
        unsafe extern "system" fn switchToControl<Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).switchToControl().into()
        }
        unsafe extern "system" fn playerDocked<Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).playerDocked(::core::mem::transmute_copy(&pbplayerdocked)).into()
        }
        unsafe extern "system" fn hasDisplay<Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).hasDisplay(::core::mem::transmute_copy(&pbhasdisplay)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlayerServices_Impl: Sized {
    fn activateUIPlugin(&mut self, bstrplugin: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPane(&mut self, bstrtaskpane: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPaneURL(&mut self, bstrtaskpane: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, bstrfriendlyname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerServices_Vtbl {
        unsafe extern "system" fn activateUIPlugin<Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).activateUIPlugin(::core::mem::transmute_copy(&bstrplugin)).into()
        }
        unsafe extern "system" fn setTaskPane<Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setTaskPane(::core::mem::transmute_copy(&bstrtaskpane)).into()
        }
        unsafe extern "system" fn setTaskPaneURL<Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setTaskPaneURL(::core::mem::transmute_copy(&bstrtaskpane), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&bstrfriendlyname)).into()
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
pub trait IWMPPlayerServices2_Impl: Sized + IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(&mut self, bstrpriority: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServices2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlayerServices2_Vtbl {
        unsafe extern "system" fn setBackgroundProcessingPriority<Impl: IWMPPlayerServices2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpriority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setBackgroundProcessingPriority(::core::mem::transmute_copy(&bstrpriority)).into()
        }
        Self {
            base: IWMPPlayerServices_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylist_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn name(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn attributeName(&mut self, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32) -> ::windows::core::Result<IWMPMedia>;
    fn getItemInfo(&mut self, bstrname: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&mut self, bstrname: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn isIdentical(&mut self, piwmpplaylist: ::core::option::Option<IWMPPlaylist>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn clear(&mut self) -> ::windows::core::Result<()>;
    fn insertItem(&mut self, lindex: i32, piwmpmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn appendItem(&mut self, piwmpmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn removeItem(&mut self, piwmpmedia: ::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn moveItem(&mut self, lindexold: i32, lindexnew: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylist_Vtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn name<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setname(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn attributeCount<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn attributeName<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrattributename)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpplaylist: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpplaylist), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn clear<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).clear().into()
        }
        unsafe extern "system" fn insertItem<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).insertItem(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn appendItem<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).appendItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn removeItem<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).removeItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn moveItem<Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).moveItem(::core::mem::transmute_copy(&lindexold), ::core::mem::transmute_copy(&lindexnew)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlaylistArray_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylistArray_Vtbl {
        unsafe extern "system" fn count<Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn newPlaylist(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getAll(&mut self) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn getByName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn remove(&mut self, pitem: ::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn setDeleted(&mut self, pitem: ::core::option::Option<IWMPPlaylist>, varfisdeleted: i16) -> ::windows::core::Result<()>;
    fn isDeleted(&mut self, pitem: ::core::option::Option<IWMPPlaylist>, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>;
    fn importPlaylist(&mut self, pitem: ::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlaylistCollection_Vtbl {
        unsafe extern "system" fn newPlaylist<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).newPlaylist(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).remove(::core::mem::transmute(&pitem)).into()
        }
        unsafe extern "system" fn setDeleted<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        unsafe extern "system" fn importPlaylist<Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, ppimporteditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).importPlaylist(::core::mem::transmute(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimporteditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPPlugin_Impl: Sized {
    fn Init(&mut self, dwplaybackcontext: usize) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn GetID(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn AdviseWMPServices(&mut self, pwmpservices: ::core::option::Option<IWMPServices>) -> ::windows::core::Result<()>;
    fn UnAdviseWMPServices(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPlugin_Vtbl {
        unsafe extern "system" fn Init<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&dwplaybackcontext)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn GetID<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn AdviseWMPServices<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpservices: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseWMPServices(::core::mem::transmute(&pwmpservices)).into()
        }
        unsafe extern "system" fn UnAdviseWMPServices<Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnAdviseWMPServices().into()
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
pub trait IWMPPluginEnable_Impl: Sized {
    fn SetEnable(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnable(&mut self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPluginEnable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPluginEnable_Vtbl {
        unsafe extern "system" fn SetEnable<Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetEnable<Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEnable(::core::mem::transmute_copy(&pfenable)).into()
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
pub trait IWMPPluginUI_Impl: Sized {
    fn SetCore(&mut self, pcore: ::core::option::Option<IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn DisplayPropertyPage(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&mut self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWMPPluginUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPPluginUI_Vtbl {
        unsafe extern "system" fn SetCore<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCore(::core::mem::transmute(&pcore)).into()
        }
        unsafe extern "system" fn Create<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&phwndwindow)).into()
        }
        unsafe extern "system" fn Destroy<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayPropertyPage(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
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
pub trait IWMPQuery_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn addCondition(&mut self, bstrattribute: super::super::Foundation::BSTR, bstroperator: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn beginNextGroup(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPQuery_Vtbl {
        unsafe extern "system" fn addCondition<Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroperator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).addCondition(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstroperator), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn beginNextGroup<Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).beginNextGroup().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            addCondition: addCondition::<Impl, IMPL_OFFSET>,
            beginNextGroup: beginNextGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPRemoteMediaServices_Impl: Sized {
    fn GetServiceType(&mut self, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetApplicationName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetScriptableObject(&mut self, pbstrname: *mut super::super::Foundation::BSTR, ppdispatch: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetCustomUIMode(&mut self, pbstrfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPRemoteMediaServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPRemoteMediaServices_Vtbl {
        unsafe extern "system" fn GetServiceType<Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetServiceType(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn GetApplicationName<Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetScriptableObject<Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScriptableObject(::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&ppdispatch)).into()
        }
        unsafe extern "system" fn GetCustomUIMode<Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCustomUIMode(::core::mem::transmute_copy(&pbstrfile)).into()
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
pub trait IWMPRenderConfig_Impl: Sized {
    fn SetinProcOnly(&mut self, finproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn inProcOnly(&mut self, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPRenderConfig_Vtbl {
        unsafe extern "system" fn SetinProcOnly<Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetinProcOnly(::core::mem::transmute_copy(&finproc)).into()
        }
        unsafe extern "system" fn inProcOnly<Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).inProcOnly(::core::mem::transmute_copy(&pfinproc)).into()
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
pub trait IWMPServices_Impl: Sized {
    fn GetStreamTime(&mut self, prt: *mut i64) -> ::windows::core::Result<()>;
    fn GetStreamState(&mut self, pstate: *mut WMPServices_StreamState) -> ::windows::core::Result<()>;
}
impl IWMPServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPServices_Vtbl {
        unsafe extern "system" fn GetStreamTime<Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamTime(::core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetStreamState<Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamState(::core::mem::transmute_copy(&pstate)).into()
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
pub trait IWMPSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn autoStart(&mut self, pfautostart: *mut i16) -> ::windows::core::Result<()>;
    fn SetautoStart(&mut self, fautostart: i16) -> ::windows::core::Result<()>;
    fn baseURL(&mut self, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetbaseURL(&mut self, bstrbaseurl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn defaultFrame(&mut self, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetdefaultFrame(&mut self, bstrdefaultframe: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn invokeURLs(&mut self, pfinvokeurls: *mut i16) -> ::windows::core::Result<()>;
    fn SetinvokeURLs(&mut self, finvokeurls: i16) -> ::windows::core::Result<()>;
    fn mute(&mut self, pfmute: *mut i16) -> ::windows::core::Result<()>;
    fn Setmute(&mut self, fmute: i16) -> ::windows::core::Result<()>;
    fn playCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetplayCount(&mut self, lcount: i32) -> ::windows::core::Result<()>;
    fn rate(&mut self, pdrate: *mut f64) -> ::windows::core::Result<()>;
    fn Setrate(&mut self, drate: f64) -> ::windows::core::Result<()>;
    fn balance(&mut self, plbalance: *mut i32) -> ::windows::core::Result<()>;
    fn Setbalance(&mut self, lbalance: i32) -> ::windows::core::Result<()>;
    fn volume(&mut self, plvolume: *mut i32) -> ::windows::core::Result<()>;
    fn Setvolume(&mut self, lvolume: i32) -> ::windows::core::Result<()>;
    fn getMode(&mut self, bstrmode: super::super::Foundation::BSTR, pvarfmode: *mut i16) -> ::windows::core::Result<()>;
    fn setMode(&mut self, bstrmode: super::super::Foundation::BSTR, varfmode: i16) -> ::windows::core::Result<()>;
    fn enableErrorDialogs(&mut self, pfenableerrordialogs: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableErrorDialogs(&mut self, fenableerrordialogs: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSettings_Vtbl {
        unsafe extern "system" fn isAvailable<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn autoStart<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautostart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).autoStart(::core::mem::transmute_copy(&pfautostart)).into()
        }
        unsafe extern "system" fn SetautoStart<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fautostart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetautoStart(::core::mem::transmute_copy(&fautostart)).into()
        }
        unsafe extern "system" fn baseURL<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).baseURL(::core::mem::transmute_copy(&pbstrbaseurl)).into()
        }
        unsafe extern "system" fn SetbaseURL<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetbaseURL(::core::mem::transmute_copy(&bstrbaseurl)).into()
        }
        unsafe extern "system" fn defaultFrame<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).defaultFrame(::core::mem::transmute_copy(&pbstrdefaultframe)).into()
        }
        unsafe extern "system" fn SetdefaultFrame<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetdefaultFrame(::core::mem::transmute_copy(&bstrdefaultframe)).into()
        }
        unsafe extern "system" fn invokeURLs<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).invokeURLs(::core::mem::transmute_copy(&pfinvokeurls)).into()
        }
        unsafe extern "system" fn SetinvokeURLs<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finvokeurls: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetinvokeURLs(::core::mem::transmute_copy(&finvokeurls)).into()
        }
        unsafe extern "system" fn mute<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).mute(::core::mem::transmute_copy(&pfmute)).into()
        }
        unsafe extern "system" fn Setmute<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setmute(::core::mem::transmute_copy(&fmute)).into()
        }
        unsafe extern "system" fn playCount<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).playCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn SetplayCount<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetplayCount(::core::mem::transmute_copy(&lcount)).into()
        }
        unsafe extern "system" fn rate<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).rate(::core::mem::transmute_copy(&pdrate)).into()
        }
        unsafe extern "system" fn Setrate<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setrate(::core::mem::transmute_copy(&drate)).into()
        }
        unsafe extern "system" fn balance<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).balance(::core::mem::transmute_copy(&plbalance)).into()
        }
        unsafe extern "system" fn Setbalance<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setbalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn volume<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).volume(::core::mem::transmute_copy(&plvolume)).into()
        }
        unsafe extern "system" fn Setvolume<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Setvolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn getMode<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getMode(::core::mem::transmute_copy(&bstrmode), ::core::mem::transmute_copy(&pvarfmode)).into()
        }
        unsafe extern "system" fn setMode<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setMode(::core::mem::transmute_copy(&bstrmode), ::core::mem::transmute_copy(&varfmode)).into()
        }
        unsafe extern "system" fn enableErrorDialogs<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).enableErrorDialogs(::core::mem::transmute_copy(&pfenableerrordialogs)).into()
        }
        unsafe extern "system" fn SetenableErrorDialogs<Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetenableErrorDialogs(::core::mem::transmute_copy(&fenableerrordialogs)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPSettings2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPSettings_Impl {
    fn defaultAudioLanguage(&mut self, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn mediaAccessRights(&mut self, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn requestMediaAccessRights(&mut self, bstrdesiredaccess: super::super::Foundation::BSTR, pvbaccepted: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSettings2_Vtbl {
        unsafe extern "system" fn defaultAudioLanguage<Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).defaultAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn mediaAccessRights<Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).mediaAccessRights(::core::mem::transmute_copy(&pbstrrights)).into()
        }
        unsafe extern "system" fn requestMediaAccessRights<Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvbaccepted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).requestMediaAccessRights(::core::mem::transmute_copy(&bstrdesiredaccess), ::core::mem::transmute_copy(&pvbaccepted)).into()
        }
        Self {
            base: IWMPSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPSkinManager_Impl: Sized {
    fn SetVisualStyle(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSkinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSkinManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSkinManager_Vtbl {
        unsafe extern "system" fn SetVisualStyle<Impl: IWMPSkinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualStyle(::core::mem::transmute_copy(&bstrpath)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetVisualStyle: SetVisualStyle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSkinManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32, pbstrstring: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPStringCollection_Vtbl {
        unsafe extern "system" fn count<Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            count: count::<Impl, IMPL_OFFSET>,
            item: item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPStringCollection_Impl {
    fn isIdentical(&mut self, piwmpstringcollection2: ::core::option::Option<IWMPStringCollection2>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, lcollectionindex: i32, bstritemname: super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeCountByType(&mut self, lcollectionindex: i32, bstrtype: super::super::Foundation::BSTR, bstrlanguage: super::super::Foundation::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&mut self, lcollectionindex: i32, bstrtype: super::super::Foundation::BSTR, bstrlanguage: super::super::Foundation::BSTR, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPStringCollection2_Vtbl {
        unsafe extern "system" fn isIdentical<Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpstringcollection2), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn getAttributeCountByType<Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getAttributeCountByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfoByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&lattributeindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base: IWMPStringCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPSubscriptionService_Impl: Sized {
    fn allowPlay(&mut self, hwnd: super::super::Foundation::HWND, pmedia: ::core::option::Option<IWMPMedia>, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowCDBurn(&mut self, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<IWMPPlaylist>, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowPDATransfer(&mut self, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<IWMPPlaylist>, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn startBackgroundProcessing(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionService_Vtbl {
        unsafe extern "system" fn allowPlay<Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: ::windows::core::RawPtr, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).allowPlay(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pmedia), ::core::mem::transmute_copy(&pfallowplay)).into()
        }
        unsafe extern "system" fn allowCDBurn<Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).allowCDBurn(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pfallowburn)).into()
        }
        unsafe extern "system" fn allowPDATransfer<Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).allowPDATransfer(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pfallowtransfer)).into()
        }
        unsafe extern "system" fn startBackgroundProcessing<Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).startBackgroundProcessing(::core::mem::transmute_copy(&hwnd)).into()
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
pub trait IWMPSubscriptionService2_Impl: Sized + IWMPSubscriptionService_Impl {
    fn stopBackgroundProcessing(&mut self) -> ::windows::core::Result<()>;
    fn serviceEvent(&mut self, event: WMPSubscriptionServiceEvent) -> ::windows::core::Result<()>;
    fn deviceAvailable(&mut self, bstrdevicename: super::super::Foundation::BSTR, pcb: ::core::option::Option<IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
    fn prepareForSync(&mut self, bstrfilename: super::super::Foundation::BSTR, bstrdevicename: super::super::Foundation::BSTR, pcb: ::core::option::Option<IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionService2_Vtbl {
        unsafe extern "system" fn stopBackgroundProcessing<Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stopBackgroundProcessing().into()
        }
        unsafe extern "system" fn serviceEvent<Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).serviceEvent(::core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn deviceAvailable<Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deviceAvailable(::core::mem::transmute_copy(&bstrdevicename), ::core::mem::transmute(&pcb)).into()
        }
        unsafe extern "system" fn prepareForSync<Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).prepareForSync(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&bstrdevicename), ::core::mem::transmute(&pcb)).into()
        }
        Self {
            base: IWMPSubscriptionService_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWMPSubscriptionServiceCallback_Impl: Sized {
    fn onComplete(&mut self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWMPSubscriptionServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSubscriptionServiceCallback_Vtbl {
        unsafe extern "system" fn onComplete<Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).onComplete(::core::mem::transmute_copy(&hrresult)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), onComplete: onComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice_Impl: Sized {
    fn friendlyName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetfriendlyName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deviceName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deviceId(&mut self, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn partnershipIndex(&mut self, plindex: *mut i32) -> ::windows::core::Result<()>;
    fn connected(&mut self, pvbconnected: *mut i16) -> ::windows::core::Result<()>;
    fn status(&mut self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()>;
    fn syncState(&mut self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()>;
    fn progress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn createPartnership(&mut self, vbshowui: i16) -> ::windows::core::Result<()>;
    fn deletePartnership(&mut self) -> ::windows::core::Result<()>;
    fn start(&mut self) -> ::windows::core::Result<()>;
    fn stop(&mut self) -> ::windows::core::Result<()>;
    fn showSettings(&mut self) -> ::windows::core::Result<()>;
    fn isIdentical(&mut self, pdevice: ::core::option::Option<IWMPSyncDevice>, pvbool: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDevice_Vtbl {
        unsafe extern "system" fn friendlyName<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).friendlyName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn SetfriendlyName<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetfriendlyName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn deviceName<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deviceName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn deviceId<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deviceId(::core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn partnershipIndex<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).partnershipIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn connected<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).connected(::core::mem::transmute_copy(&pvbconnected)).into()
        }
        unsafe extern "system" fn status<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).status(::core::mem::transmute_copy(&pwmpds)).into()
        }
        unsafe extern "system" fn syncState<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).syncState(::core::mem::transmute_copy(&pwmpss)).into()
        }
        unsafe extern "system" fn progress<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn getItemInfo<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn createPartnership<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbshowui: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).createPartnership(::core::mem::transmute_copy(&vbshowui)).into()
        }
        unsafe extern "system" fn deletePartnership<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deletePartnership().into()
        }
        unsafe extern "system" fn start<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).start().into()
        }
        unsafe extern "system" fn stop<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).stop().into()
        }
        unsafe extern "system" fn showSettings<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).showSettings().into()
        }
        unsafe extern "system" fn isIdentical<Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pvbool)).into()
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
pub trait IWMPSyncDevice2_Impl: Sized + IWMPSyncDevice_Impl {
    fn setItemInfo(&mut self, bstritemname: super::super::Foundation::BSTR, bstrval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDevice2_Vtbl {
        unsafe extern "system" fn setItemInfo<Impl: IWMPSyncDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&bstrval)).into()
        }
        Self { base: IWMPSyncDevice_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), setItemInfo: setItemInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSyncDevice3_Impl: Sized + IWMPSyncDevice_Impl + IWMPSyncDevice2_Impl {
    fn estimateSyncSize(&mut self, pnonruleplaylist: ::core::option::Option<IWMPPlaylist>, prulesplaylist: ::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cancelEstimation(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSyncDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncDevice3_Vtbl {
        unsafe extern "system" fn estimateSyncSize<Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: ::windows::core::RawPtr, prulesplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).estimateSyncSize(::core::mem::transmute(&pnonruleplaylist), ::core::mem::transmute(&prulesplaylist)).into()
        }
        unsafe extern "system" fn cancelEstimation<Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).cancelEstimation().into()
        }
        Self {
            base: IWMPSyncDevice2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Impl, IMPL_OFFSET>,
            cancelEstimation: cancelEstimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPSyncServices_Impl: Sized {
    fn deviceCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getDevice(&mut self, lindex: i32) -> ::windows::core::Result<IWMPSyncDevice>;
}
impl IWMPSyncServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPSyncServices_Vtbl {
        unsafe extern "system" fn deviceCount<Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).deviceCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getDevice<Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getDevice(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IWMPTranscodePolicy_Impl: Sized {
    fn allowTranscode(&mut self, pvballow: *mut i16) -> ::windows::core::Result<()>;
}
impl IWMPTranscodePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPTranscodePolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPTranscodePolicy_Vtbl {
        unsafe extern "system" fn allowTranscode<Impl: IWMPTranscodePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvballow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).allowTranscode(::core::mem::transmute_copy(&pvballow)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), allowTranscode: allowTranscode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPTranscodePolicy as ::windows::core::Interface>::IID
    }
}
pub trait IWMPUserEventSink_Impl: Sized {
    fn NotifyUserEvent(&mut self, eventcode: i32) -> ::windows::core::Result<()>;
}
impl IWMPUserEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPUserEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPUserEventSink_Vtbl {
        unsafe extern "system" fn NotifyUserEvent<Impl: IWMPUserEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyUserEvent(::core::mem::transmute_copy(&eventcode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NotifyUserEvent: NotifyUserEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPUserEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfig_Impl: Sized {
    fn SetpresenterActivate(&mut self, pactivate: ::core::option::Option<super::MediaFoundation::IMFActivate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IWMPVideoRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPVideoRenderConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPVideoRenderConfig_Vtbl {
        unsafe extern "system" fn SetpresenterActivate<Impl: IWMPVideoRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetpresenterActivate(::core::mem::transmute(&pactivate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetpresenterActivate: SetpresenterActivate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPVideoRenderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPWindowMessageSink_Impl: Sized {
    fn OnWindowMessage(&mut self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPWindowMessageSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPWindowMessageSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPWindowMessageSink_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Impl: IWMPWindowMessageSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowMessage(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnWindowMessage: OnWindowMessage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPWindowMessageSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed_Impl: Sized {
    fn Xml(&mut self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Rename(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetUrl(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Move(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Parent(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LastWriteTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Download(&mut self) -> ::windows::core::Result<()>;
    fn AsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn SyncSetting(&mut self) -> ::windows::core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&mut self, fss: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<u32>;
    fn SetInterval(&mut self, uiinterval: u32) -> ::windows::core::Result<()>;
    fn LastDownloadTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn LocalEnclosurePath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Items(&mut self) -> ::windows::core::Result<IXFeedsEnum>;
    fn GetItem(&mut self, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn MarkAllItemsRead(&mut self) -> ::windows::core::Result<()>;
    fn MaxItemCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxItemCount(&mut self, uimaxitemcount: u32) -> ::windows::core::Result<()>;
    fn DownloadEnclosuresAutomatically(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetDownloadEnclosuresAutomatically(&mut self, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DownloadStatus(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&mut self, pstream: ::core::option::Option<super::super::System::Com::IStream>, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Link(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn LastBuildDate(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn PubDate(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Ttl(&mut self) -> ::windows::core::Result<u32>;
    fn Language(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Copyright(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsList(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UnreadItemCount(&mut self) -> ::windows::core::Result<u32>;
    fn ItemCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeed_Vtbl {
        unsafe extern "system" fn Xml<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&uiitemcount), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Url<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn LocalId<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastWriteTime<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastwritetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Download<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *pfss = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncSetting(::core::mem::transmute_copy(&fss)).into()
        }
        unsafe extern "system" fn Interval<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *puiinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItem(::core::mem::transmute_copy(&uiid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn MarkAllItemsRead<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MarkAllItemsRead().into()
        }
        unsafe extern "system" fn MaxItemCount<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puimaxitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxItemCount(::core::mem::transmute_copy(&uimaxitemcount)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdownloadenclosuresautomatically = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&bdownloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *pfde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Merge(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszimageurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastbuilddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstpubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    *puittl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlanguage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcopyright = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsList() {
                ::core::result::Result::Ok(ok__) => {
                    *pbislist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn UnreadItemCount<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puiunreaditemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puiitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IXFeed2_Impl: Sized + IXFeed_Impl {
    fn GetItemByEffectiveId(&mut self, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LastItemDownloadTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Username(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetCredentials(&mut self, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ClearCredentials(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeed2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemByEffectiveId(::core::mem::transmute_copy(&uieffectiveid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastItemDownloadTime<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastitemdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpassword = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        unsafe extern "system" fn ClearCredentials<Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearCredentials().into()
        }
        Self {
            base: IXFeed_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXFeedEnclosure_Impl: Sized {
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Type(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn AsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&mut self) -> ::windows::core::Result<()>;
    fn DownloadStatus(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&mut self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Parent(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn DownloadMimeType(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn RemoveFile(&mut self) -> ::windows::core::Result<()>;
    fn SetFile(&mut self, pszdownloadurl: super::super::Foundation::PWSTR, pszdownloadfilepath: super::super::Foundation::PWSTR, pszdownloadmimetype: super::super::Foundation::PWSTR, pszenclosurefilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedEnclosure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszmimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *puilength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *pfde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszmimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdownloadurl: super::super::Foundation::PWSTR, pszdownloadfilepath: super::super::Foundation::PWSTR, pszdownloadmimetype: super::super::Foundation::PWSTR, pszenclosurefilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFile(::core::mem::transmute_copy(&pszdownloadurl), ::core::mem::transmute_copy(&pszdownloadfilepath), ::core::mem::transmute_copy(&pszdownloadmimetype), ::core::mem::transmute_copy(&pszenclosurefilename)).into()
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
pub trait IXFeedEvents_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedMoved(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMoved<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMoved(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
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
pub trait IXFeedFolder_Impl: Sized {
    fn Feeds(&mut self) -> ::windows::core::Result<IXFeedsEnum>;
    fn Subfolders(&mut self) -> ::windows::core::Result<IXFeedsEnum>;
    fn CreateFeed(&mut self, pszname: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateSubfolder(&mut self, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExistsFeed(&mut self, pszname: super::super::Foundation::PWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ExistsSubfolder(&mut self, pszname: super::super::Foundation::PWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFeed(&mut self, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSubfolder(&mut self, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Rename(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Move(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Parent(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsRoot(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn TotalUnreadItemCount(&mut self) -> ::windows::core::Result<u32>;
    fn TotalItemCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateSubfolder<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExistsFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pbfeedexists)).into()
        }
        unsafe extern "system" fn ExistsSubfolder<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExistsSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pbsubfolderexists)).into()
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetSubfolder<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Name<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Path<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRoot<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisrootfeedfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puitotalunreaditemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puitotalitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IXFeedFolderEvents_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FolderAdded(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FolderDeleted(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FolderRenamed(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FolderMovedFrom(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FolderMovedTo(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FolderItemCountChanged(&mut self, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
    fn FeedAdded(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedMovedFrom(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedMovedTo(&mut self, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedFolderEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FolderAdded<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderAdded(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FolderDeleted<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FolderRenamed<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderMovedFrom(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderMovedTo(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FolderItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        unsafe extern "system" fn FeedAdded<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedAdded(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDeleted<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMovedFrom(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedMovedTo(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
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
pub trait IXFeedItem_Impl: Sized {
    fn Xml(&mut self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Link(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Guid(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn PubDate(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Comments(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Author(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Enclosure(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsRead(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsRead(&mut self, bisread: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<u32>;
    fn Parent(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn LastDownloadTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Modified(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&fxif)) {
                ::core::result::Result::Ok(ok__) => {
                    *pps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszguid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstpubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszauthor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enclosure(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRead<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRead(::core::mem::transmute_copy(&bisread)).into()
        }
        unsafe extern "system" fn LocalId<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *puiid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pstmodifiedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait IXFeedItem2_Impl: Sized + IXFeedItem_Impl {
    fn EffectiveId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Impl: IXFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    *puieffectiveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IXFeedItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EffectiveId: EffectiveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedItem2 as ::windows::core::Interface>::IID
    }
}
pub trait IXFeedsEnum_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IXFeedsEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnum_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *puicount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Count: Count::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedsEnum as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedsManager_Impl: Sized {
    fn RootFolder(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsSubscribed(&mut self, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ExistsFeed(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFeed(&mut self, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFeedByUrl(&mut self, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExistsFolder(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFolder(&mut self, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DeleteFeed(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteFolder(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn BackgroundSync(&mut self, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()>;
    fn BackgroundSyncStatus(&mut self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetDefaultInterval(&mut self, uiinterval: u32) -> ::windows::core::Result<()>;
    fn AsyncSyncAll(&mut self) -> ::windows::core::Result<()>;
    fn Normalize(&mut self, pstreamin: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn ItemCountLimit(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RootFolder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsSubscribed<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscribed(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsubscribed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfeedexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetFeedByUrl<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeedByUrl(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFolder<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistsFolder(::core::mem::transmute_copy(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfolderexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFolder(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DeleteFeed<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFeed(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackgroundSync(::core::mem::transmute_copy(&fbsa)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfbss = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *puiinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamin: ::windows::core::RawPtr, ppstreamout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normalize(::core::mem::transmute(&pstreamin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstreamout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *puiitemcountlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait _WMPOCXEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _WMPOCXEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _WMPOCXEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _WMPOCXEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_WMPOCXEvents as ::windows::core::Interface>::IID
    }
}
