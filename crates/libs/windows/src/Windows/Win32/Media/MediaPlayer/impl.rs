#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&mut self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rename(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUrl(&mut self, feedurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, newparentpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn Merge(&mut self, feedxml: &super::super::Foundation::BSTR, feedurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsList(&mut self) -> ::windows::core::Result<i16>;
    fn MarkAllItemsRead(&mut self) -> ::windows::core::Result<()>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn UnreadItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn ItemCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>() -> IFeed_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *feedurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&feedurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *feedguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *syncsetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyncSetting(::core::mem::transmute_copy(&syncsetting)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&itemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homepage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *homepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *imageurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    *lastbuilddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *lastpopulatedate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    *ttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyright: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    *copyright = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxItemCount(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    *downloadenclosuresautomatically = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&downloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Merge(::core::mem::transmute_copy(&feedxml), ::core::mem::transmute_copy(&feedurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *feedurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsList() {
                ::core::result::Result::Ok(ok__) => {
                    *islist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MarkAllItemsRead().into()
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Xml: Xml::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            SyncSetting: SyncSetting::<Identity, Impl, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, Impl, OFFSET>,
            Items: Items::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Ttl: Ttl::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Copyright: Copyright::<Identity, Impl, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, Impl, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, Impl, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            IsList: IsList::<Identity, Impl, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeed as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFeed_Impl {
    fn GetItemByEffectiveId(&mut self, itemeffectiveid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn LastItemDownloadTime(&mut self) -> ::windows::core::Result<f64>;
    fn Username(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCredentials(&mut self, username: &super::super::Foundation::BSTR, password: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClearCredentials(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>() -> IFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemByEffectiveId(::core::mem::transmute_copy(&itemeffectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastitemdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Username() {
                ::core::result::Result::Ok(ok__) => {
                    *username = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *password = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearCredentials().into()
        }
        Self {
            base: IFeed_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeed2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFeed as ::windows::core::Interface>::IID
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
    fn SetFile(&mut self, downloadurl: &super::super::Foundation::BSTR, downloadfilepath: &super::super::Foundation::BSTR, downloadmimetype: &super::super::Foundation::BSTR, enclosurefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEnclosure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>() -> IFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosureurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *mimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *localpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosureurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *mimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, downloadmimetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enclosurefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFile(::core::mem::transmute_copy(&downloadurl), ::core::mem::transmute_copy(&downloadfilepath), ::core::mem::transmute_copy(&downloadmimetype), ::core::mem::transmute_copy(&enclosurefilename)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Url: Url::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            LocalPath: LocalPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, Impl, OFFSET>,
            RemoveFile: RemoveFile::<Identity, Impl, OFFSET>,
            SetFile: SetFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedEnclosure as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMoved(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, path: &super::super::Foundation::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, path: &super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>() -> IFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMoved(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Error: Error::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMoved: FeedMoved::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Feeds(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Subfolders(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateFeed(&mut self, feedname: &super::super::Foundation::BSTR, feedurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateSubfolder(&mut self, foldername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFeed(&mut self, feedname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFeed(&mut self, feedname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsSubfolder(&mut self, foldername: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetSubfolder(&mut self, foldername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rename(&mut self, foldername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, newparentpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsRoot(&mut self) -> ::windows::core::Result<i16>;
    fn TotalUnreadItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn TotalItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetWatcher(&mut self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>() -> IFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFeed(::core::mem::transmute_copy(&feedname), ::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeed(::core::mem::transmute_copy(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubfolder(::core::mem::transmute_copy(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *foldername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&foldername)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *folderpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isroot: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *isroot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Feeds: Feeds::<Identity, Impl, OFFSET>,
            Subfolders: Subfolders::<Identity, Impl, OFFSET>,
            CreateFeed: CreateFeed::<Identity, Impl, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, Impl, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            IsRoot: IsRoot::<Identity, Impl, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, Impl, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedFolder as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolderEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&mut self) -> ::windows::core::Result<()>;
    fn FolderAdded(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderDeleted(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderRenamed(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedFrom(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedTo(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FolderItemCountChanged(&mut self, path: &super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
    fn FeedAdded(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDeleted(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedFrom(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedTo(&mut self, path: &super::super::Foundation::BSTR, oldpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&mut self, path: &super::super::Foundation::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&mut self, path: &super::super::Foundation::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolderEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>() -> IFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderAdded(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderMovedFrom(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderMovedTo(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedAdded(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMovedFrom(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oldpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMovedTo(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Error: Error::<Identity, Impl, OFFSET>,
            FolderAdded: FolderAdded::<Identity, Impl, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, Impl, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, Impl, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, Impl, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, Impl, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, Impl, OFFSET>,
            FeedAdded: FeedAdded::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, Impl, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedFolderEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>() -> IFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linkurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *linkurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *itemguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Comments() {
                ::core::result::Result::Ok(ok__) => {
                    *comments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *author = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enclosure() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *isread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsRead(::core::mem::transmute_copy(&isread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *itemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *itemurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *modified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Xml: Xml::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Comments: Comments::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Enclosure: Enclosure::<Identity, Impl, OFFSET>,
            IsRead: IsRead::<Identity, Impl, OFFSET>,
            SetIsRead: SetIsRead::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFeedItem_Impl {
    fn EffectiveId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem2_Impl, const OFFSET: isize>() -> IFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Identity: ::windows::core::IUnknownImpl, Impl: IFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    *effectiveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFeedItem_Vtbl::new::<Identity, Impl, OFFSET>(), EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IFeedItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnum_Impl, const OFFSET: isize>() -> IFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedsEnum as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RootFolder(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsSubscribed(&mut self, feedurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn ExistsFeed(&mut self, feedpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFeed(&mut self, feedpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetFeedByUrl(&mut self, feedurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFolder(&mut self, folderpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetFolder(&mut self, folderpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DeleteFeed(&mut self, feedpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteFolder(&mut self, folderpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BackgroundSync(&mut self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()>;
    fn BackgroundSyncStatus(&mut self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultInterval(&mut self, minutes: i32) -> ::windows::core::Result<()>;
    fn AsyncSyncAll(&mut self) -> ::windows::core::Result<()>;
    fn Normalize(&mut self, feedxmlin: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ItemCountLimit(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>() -> IFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subscribed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSubscribed(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *subscribed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeed(::core::mem::transmute_copy(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeedByUrl(::core::mem::transmute_copy(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsFolder(::core::mem::transmute_copy(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *exists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, disp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *disp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFeed(::core::mem::transmute_copy(&feedpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&folderpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundSync(::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxmlin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, feedxmlout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Normalize(::core::mem::transmute_copy(&feedxmlin)) {
                ::core::result::Result::Ok(ok__) => {
                    *feedxmlout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *itemcountlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RootFolder: RootFolder::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, Impl, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, Impl, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, Impl, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, Impl, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, Impl, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedsManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPAudioRenderConfig_Impl: Sized {
    fn audioOutputDevice(&mut self, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetaudioOutputDevice(&mut self, bstroutputdevice: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPAudioRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>() -> IWMPAudioRenderConfig_Vtbl {
        unsafe extern "system" fn audioOutputDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).audioOutputDevice(::core::mem::transmute_copy(&pbstroutputdevice)).into()
        }
        unsafe extern "system" fn SetaudioOutputDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetaudioOutputDevice(::core::mem::transmute_copy(&bstroutputdevice)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            audioOutputDevice: audioOutputDevice::<Identity, Impl, OFFSET>,
            SetaudioOutputDevice: SetaudioOutputDevice::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdrom_Impl, const OFFSET: isize>() -> IWMPCdrom_Vtbl {
        unsafe extern "system" fn driveSpecifier<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).driveSpecifier(::core::mem::transmute_copy(&pbstrdrive)).into()
        }
        unsafe extern "system" fn playlist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).playlist() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eject<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).eject().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            driveSpecifier: driveSpecifier::<Identity, Impl, OFFSET>,
            playlist: playlist::<Identity, Impl, OFFSET>,
            eject: eject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdrom as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPCdromBurn_Impl: Sized {
    fn isAvailable(&mut self, bstritem: &super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritem: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn label(&mut self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setlabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn burnFormat(&mut self, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::Result<()>;
    fn SetburnFormat(&mut self, wmpbf: WMPBurnFormat) -> ::windows::core::Result<()>;
    fn burnPlaylist(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetburnPlaylist(&mut self, pplaylist: &::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn refreshStatus(&mut self) -> ::windows::core::Result<()>;
    fn burnState(&mut self, pwmpbs: *mut WMPBurnState) -> ::windows::core::Result<()>;
    fn burnProgress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startBurn(&mut self) -> ::windows::core::Result<()>;
    fn stopBurn(&mut self) -> ::windows::core::Result<()>;
    fn erase(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPCdromBurn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>() -> IWMPCdromBurn_Vtbl {
        unsafe extern "system" fn isAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn label<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).label(::core::mem::transmute_copy(&pbstrlabel)).into()
        }
        unsafe extern "system" fn Setlabel<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setlabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn burnFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).burnFormat(::core::mem::transmute_copy(&pwmpbf)).into()
        }
        unsafe extern "system" fn SetburnFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetburnFormat(::core::mem::transmute_copy(&wmpbf)).into()
        }
        unsafe extern "system" fn burnPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).burnPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetburnPlaylist(::core::mem::transmute(&pplaylist)).into()
        }
        unsafe extern "system" fn refreshStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).refreshStatus().into()
        }
        unsafe extern "system" fn burnState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).burnState(::core::mem::transmute_copy(&pwmpbs)).into()
        }
        unsafe extern "system" fn burnProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).burnProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startBurn<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startBurn().into()
        }
        unsafe extern "system" fn stopBurn<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stopBurn().into()
        }
        unsafe extern "system" fn erase<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).erase().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            isAvailable: isAvailable::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            label: label::<Identity, Impl, OFFSET>,
            Setlabel: Setlabel::<Identity, Impl, OFFSET>,
            burnFormat: burnFormat::<Identity, Impl, OFFSET>,
            SetburnFormat: SetburnFormat::<Identity, Impl, OFFSET>,
            burnPlaylist: burnPlaylist::<Identity, Impl, OFFSET>,
            SetburnPlaylist: SetburnPlaylist::<Identity, Impl, OFFSET>,
            refreshStatus: refreshStatus::<Identity, Impl, OFFSET>,
            burnState: burnState::<Identity, Impl, OFFSET>,
            burnProgress: burnProgress::<Identity, Impl, OFFSET>,
            startBurn: startBurn::<Identity, Impl, OFFSET>,
            stopBurn: stopBurn::<Identity, Impl, OFFSET>,
            erase: erase::<Identity, Impl, OFFSET>,
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
    fn getByDriveSpecifier(&mut self, bstrdrivespecifier: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPCdrom>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdromCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>() -> IWMPCdromCollection_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcdrom: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByDriveSpecifier(::core::mem::transmute_copy(&bstrdrivespecifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcdrom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWMPCdromRip_Impl: Sized {
    fn ripState(&mut self, pwmprs: *mut WMPRipState) -> ::windows::core::Result<()>;
    fn ripProgress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startRip(&mut self) -> ::windows::core::Result<()>;
    fn stopRip(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPCdromRip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const OFFSET: isize>() -> IWMPCdromRip_Vtbl {
        unsafe extern "system" fn ripState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ripState(::core::mem::transmute_copy(&pwmprs)).into()
        }
        unsafe extern "system" fn ripProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ripProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startRip<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startRip().into()
        }
        unsafe extern "system" fn stopRip<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stopRip().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ripState: ripState::<Identity, Impl, OFFSET>,
            ripProgress: ripProgress::<Identity, Impl, OFFSET>,
            startRip: startRip::<Identity, Impl, OFFSET>,
            stopRip: stopRip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromRip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPClosedCaption_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SAMIStyle(&mut self, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIStyle(&mut self, bstrsamistyle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SAMILang(&mut self, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMILang(&mut self, bstrsamilang: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SAMIFileName(&mut self, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIFileName(&mut self, bstrsamifilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn captioningId(&mut self, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetcaptioningId(&mut self, bstrcaptioningid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>() -> IWMPClosedCaption_Vtbl {
        unsafe extern "system" fn SAMIStyle<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAMIStyle(::core::mem::transmute_copy(&pbstrsamistyle)).into()
        }
        unsafe extern "system" fn SetSAMIStyle<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSAMIStyle(::core::mem::transmute_copy(&bstrsamistyle)).into()
        }
        unsafe extern "system" fn SAMILang<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAMILang(::core::mem::transmute_copy(&pbstrsamilang)).into()
        }
        unsafe extern "system" fn SetSAMILang<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSAMILang(::core::mem::transmute_copy(&bstrsamilang)).into()
        }
        unsafe extern "system" fn SAMIFileName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAMIFileName(::core::mem::transmute_copy(&pbstrsamifilename)).into()
        }
        unsafe extern "system" fn SetSAMIFileName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSAMIFileName(::core::mem::transmute_copy(&bstrsamifilename)).into()
        }
        unsafe extern "system" fn captioningId<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).captioningId(::core::mem::transmute_copy(&pbstrcaptioningid)).into()
        }
        unsafe extern "system" fn SetcaptioningId<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcaptioningId(::core::mem::transmute_copy(&bstrcaptioningid)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SAMIStyle: SAMIStyle::<Identity, Impl, OFFSET>,
            SetSAMIStyle: SetSAMIStyle::<Identity, Impl, OFFSET>,
            SAMILang: SAMILang::<Identity, Impl, OFFSET>,
            SetSAMILang: SetSAMILang::<Identity, Impl, OFFSET>,
            SAMIFileName: SAMIFileName::<Identity, Impl, OFFSET>,
            SetSAMIFileName: SetSAMIFileName::<Identity, Impl, OFFSET>,
            captioningId: captioningId::<Identity, Impl, OFFSET>,
            SetcaptioningId: SetcaptioningId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPClosedCaption as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>() -> IWMPClosedCaption2_Vtbl {
        unsafe extern "system" fn SAMILangCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAMILangCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMILangName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getSAMILangName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn getSAMILangID<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getSAMILangID(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SAMIStyleCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SAMIStyleCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMIStyleName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getSAMIStyleName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        Self {
            base: IWMPClosedCaption_Vtbl::new::<Identity, Impl, OFFSET>(),
            SAMILangCount: SAMILangCount::<Identity, Impl, OFFSET>,
            getSAMILangName: getSAMILangName::<Identity, Impl, OFFSET>,
            getSAMILangID: getSAMILangID::<Identity, Impl, OFFSET>,
            SAMIStyleCount: SAMIStyleCount::<Identity, Impl, OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPClosedCaption2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPClosedCaption as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>() -> IWMPContentContainer_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrice() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPrice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentPrice(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentID<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentID(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetPrice: GetPrice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContentCount: GetContentCount::<Identity, Impl, OFFSET>,
            GetContentPrice: GetContentPrice::<Identity, Impl, OFFSET>,
            GetContentID: GetContentID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>() -> IWMPContentContainerList_Vtbl {
        unsafe extern "system" fn GetTransactionType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTransactionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pwmptt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccontainer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainer(::core::mem::transmute_copy(&idxcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTransactionType: GetTransactionType::<Identity, Impl, OFFSET>,
            GetContainerCount: GetContainerCount::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentContainerList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPContentPartner_Impl: Sized {
    fn SetCallback(&mut self, pcallback: &::core::option::Option<IWMPContentPartnerCallback>) -> ::windows::core::Result<()>;
    fn Notify(&mut self, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetItemInfo(&mut self, bstrinfoname: &super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetContentPartnerInfo(&mut self, bstrinfoname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCommands(&mut self, location: &super::super::Foundation::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &super::super::Foundation::BSTR, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::Result<()>;
    fn InvokeCommand(&mut self, dwcommandid: u32, location: &super::super::Foundation::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &super::super::Foundation::BSTR, citemids: u32, rgitemids: *const u32) -> ::windows::core::Result<()>;
    fn CanBuySilent(&mut self, pinfo: &::core::option::Option<IWMPContentContainerList>, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::Result<()>;
    fn Buy(&mut self, pinfo: &::core::option::Option<IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn GetStreamingURL(&mut self, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Download(&mut self, pinfo: &::core::option::Option<IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn DownloadTrackComplete(&mut self, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RefreshLicense(&mut self, dwcookie: u32, flocal: i16, bstrurl: &super::super::Foundation::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: &super::super::Foundation::BSTR, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCatalogURL(&mut self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetTemplate(&mut self, task: WMPTaskType, location: &super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, clicklocation: &super::super::Foundation::BSTR, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: &super::super::Foundation::BSTR, bstrviewparams: &super::super::Foundation::BSTR, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::Result<()>;
    fn UpdateDevice(&mut self, bstrdevicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetListContents(&mut self, location: &super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: &super::super::Foundation::BSTR, bstrparams: &super::super::Foundation::BSTR, dwlistcookie: u32) -> ::windows::core::Result<()>;
    fn Login(&mut self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::Result<()>;
    fn Authenticate(&mut self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB) -> ::windows::core::Result<()>;
    fn Logout(&mut self) -> ::windows::core::Result<()>;
    fn SendMessage(&mut self, bstrmsg: &super::super::Foundation::BSTR, bstrparam: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StationEvent(&mut self, bstrstationeventtype: &super::super::Foundation::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: &super::super::Foundation::BSTR, dwsecondsplayed: u32) -> ::windows::core::Result<()>;
    fn CompareContainerListPrices(&mut self, plistbase: &::core::option::Option<IWMPContentContainerList>, plistcompare: &::core::option::Option<IWMPContentContainerList>) -> ::windows::core::Result<i32>;
    fn VerifyPermission(&mut self, bstrpermission: &super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>() -> IWMPContentPartner_Vtbl {
        unsafe extern "system" fn SetCallback<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallback(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn GetItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemInfo(::core::mem::transmute_copy(&bstrinfoname), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPartnerInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentPartnerInfo(::core::mem::transmute_copy(&bstrinfoname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCommands(::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute_copy(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&prgitemids), ::core::mem::transmute_copy(&pcitemids), ::core::mem::transmute_copy(&pprgitems)).into()
        }
        unsafe extern "system" fn InvokeCommand<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeCommand(::core::mem::transmute_copy(&dwcommandid), ::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute_copy(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&rgitemids)).into()
        }
        unsafe extern "system" fn CanBuySilent<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, pbstrtotalprice: *mut super::super::Foundation::BSTR, psilentok: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanBuySilent(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&pbstrtotalprice), ::core::mem::transmute_copy(&psilentok)).into()
        }
        unsafe extern "system" fn Buy<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Buy(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetStreamingURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamingURL(::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&pstreamcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn DownloadTrackComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadTrackComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&downloadtrackparam)).into()
        }
        unsafe extern "system" fn RefreshLicense<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: i16, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshLicense(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&flocal), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&bstrrefreshreason), ::core::mem::transmute_copy(&preasoncontext)).into()
        }
        unsafe extern "system" fn GetCatalogURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut super::super::Foundation::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCatalogURL(::core::mem::transmute_copy(&dwcatalogversion), ::core::mem::transmute_copy(&dwcatalogschemaversion), ::core::mem::transmute_copy(&cataloglcid), ::core::mem::transmute_copy(&pdwnewcatalogversion), ::core::mem::transmute_copy(&pbstrcatalogurl), ::core::mem::transmute_copy(&pexpirationdate)).into()
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, clicklocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrviewparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrtemplateurl: *mut super::super::Foundation::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTemplate(::core::mem::transmute_copy(&task), ::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&clicklocation), ::core::mem::transmute_copy(&pclickcontext), ::core::mem::transmute_copy(&bstrfilter), ::core::mem::transmute_copy(&bstrviewparams), ::core::mem::transmute_copy(&pbstrtemplateurl), ::core::mem::transmute_copy(&ptemplatesize)).into()
        }
        unsafe extern "system" fn UpdateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateDevice(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn GetListContents<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetListContents(::core::mem::transmute_copy(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&bstrlisttype), ::core::mem::transmute_copy(&bstrparams), ::core::mem::transmute_copy(&dwlistcookie)).into()
        }
        unsafe extern "system" fn Login<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: i16, foktocache: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Login(::core::mem::transmute_copy(&userinfo), ::core::mem::transmute_copy(&pwdinfo), ::core::mem::transmute_copy(&fusedcachedcreds), ::core::mem::transmute_copy(&foktocache)).into()
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Authenticate(::core::mem::transmute_copy(&userinfo), ::core::mem::transmute_copy(&pwdinfo)).into()
        }
        unsafe extern "system" fn Logout<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logout().into()
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendMessage(::core::mem::transmute_copy(&bstrmsg), ::core::mem::transmute_copy(&bstrparam)).into()
        }
        unsafe extern "system" fn StationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StationEvent(::core::mem::transmute_copy(&bstrstationeventtype), ::core::mem::transmute_copy(&stationid), ::core::mem::transmute_copy(&playlistindex), ::core::mem::transmute_copy(&trackid), ::core::mem::transmute_copy(&trackdata), ::core::mem::transmute_copy(&dwsecondsplayed)).into()
        }
        unsafe extern "system" fn CompareContainerListPrices<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistbase: ::windows::core::RawPtr, plistcompare: ::windows::core::RawPtr, presult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareContainerListPrices(::core::mem::transmute(&plistbase), ::core::mem::transmute(&plistcompare)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermission<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VerifyPermission(::core::mem::transmute_copy(&bstrpermission), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, Impl, OFFSET>,
            GetContentPartnerInfo: GetContentPartnerInfo::<Identity, Impl, OFFSET>,
            GetCommands: GetCommands::<Identity, Impl, OFFSET>,
            InvokeCommand: InvokeCommand::<Identity, Impl, OFFSET>,
            CanBuySilent: CanBuySilent::<Identity, Impl, OFFSET>,
            Buy: Buy::<Identity, Impl, OFFSET>,
            GetStreamingURL: GetStreamingURL::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            DownloadTrackComplete: DownloadTrackComplete::<Identity, Impl, OFFSET>,
            RefreshLicense: RefreshLicense::<Identity, Impl, OFFSET>,
            GetCatalogURL: GetCatalogURL::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            UpdateDevice: UpdateDevice::<Identity, Impl, OFFSET>,
            GetListContents: GetListContents::<Identity, Impl, OFFSET>,
            Login: Login::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            Logout: Logout::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            StationEvent: StationEvent::<Identity, Impl, OFFSET>,
            CompareContainerListPrices: CompareContainerListPrices::<Identity, Impl, OFFSET>,
            VerifyPermission: VerifyPermission::<Identity, Impl, OFFSET>,
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
    fn DownloadTrack(&mut self, cookie: u32, bstrtrackurl: &super::super::Foundation::BSTR, dwservicetrackid: u32, bstrdownloadparams: &super::super::Foundation::BSTR, hrdownload: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetCatalogVersion(&mut self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn UpdateDeviceComplete(&mut self, bstrdevicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangeView(&mut self, bstrtype: &super::super::Foundation::BSTR, bstrid: &super::super::Foundation::BSTR, bstrfilter: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddListContents(&mut self, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::Result<()>;
    fn ListContentsComplete(&mut self, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn SendMessageComplete(&mut self, bstrmsg: &super::super::Foundation::BSTR, bstrparam: &super::super::Foundation::BSTR, bstrresult: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetContentIDsInLibrary(&mut self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::Result<()>;
    fn RefreshLicenseComplete(&mut self, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ShowPopup(&mut self, lindex: i32, bstrparameters: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn VerifyPermissionComplete(&mut self, bstrpermission: &super::super::Foundation::BSTR, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartnerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>() -> IWMPContentPartnerCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn BuyComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BuyComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwbuycookie)).into()
        }
        unsafe extern "system" fn DownloadTrack<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadTrack(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&bstrtrackurl), ::core::mem::transmute_copy(&dwservicetrackid), ::core::mem::transmute_copy(&bstrdownloadparams), ::core::mem::transmute_copy(&hrdownload)).into()
        }
        unsafe extern "system" fn GetCatalogVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCatalogVersion(::core::mem::transmute_copy(&pdwversion), ::core::mem::transmute_copy(&pdwschemaversion), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn UpdateDeviceComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateDeviceComplete(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn ChangeView<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeView(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrid), ::core::mem::transmute_copy(&bstrfilter)).into()
        }
        unsafe extern "system" fn AddListContents<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddListContents(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&citems), ::core::mem::transmute_copy(&prgitems)).into()
        }
        unsafe extern "system" fn ListContentsComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ListContentsComplete(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&hrsuccess)).into()
        }
        unsafe extern "system" fn SendMessageComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendMessageComplete(::core::mem::transmute_copy(&bstrmsg), ::core::mem::transmute_copy(&bstrparam), ::core::mem::transmute_copy(&bstrresult)).into()
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContentIDsInLibrary(::core::mem::transmute_copy(&pccontentids), ::core::mem::transmute_copy(&pprgids)).into()
        }
        unsafe extern "system" fn RefreshLicenseComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshLicenseComplete(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&hrrefresh)).into()
        }
        unsafe extern "system" fn ShowPopup<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowPopup(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&bstrparameters)).into()
        }
        unsafe extern "system" fn VerifyPermissionComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VerifyPermissionComplete(::core::mem::transmute_copy(&bstrpermission), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrpermission)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Notify: Notify::<Identity, Impl, OFFSET>,
            BuyComplete: BuyComplete::<Identity, Impl, OFFSET>,
            DownloadTrack: DownloadTrack::<Identity, Impl, OFFSET>,
            GetCatalogVersion: GetCatalogVersion::<Identity, Impl, OFFSET>,
            UpdateDeviceComplete: UpdateDeviceComplete::<Identity, Impl, OFFSET>,
            ChangeView: ChangeView::<Identity, Impl, OFFSET>,
            AddListContents: AddListContents::<Identity, Impl, OFFSET>,
            ListContentsComplete: ListContentsComplete::<Identity, Impl, OFFSET>,
            SendMessageComplete: SendMessageComplete::<Identity, Impl, OFFSET>,
            GetContentIDsInLibrary: GetContentIDsInLibrary::<Identity, Impl, OFFSET>,
            RefreshLicenseComplete: RefreshLicenseComplete::<Identity, Impl, OFFSET>,
            ShowPopup: ShowPopup::<Identity, Impl, OFFSET>,
            VerifyPermissionComplete: VerifyPermissionComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentPartnerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: &super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
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
    fn SetcurrentItem(&mut self, piwmpmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn currentMarker(&mut self, plmarker: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentMarker(&mut self, lmarker: i32) -> ::windows::core::Result<()>;
    fn playItem(&mut self, piwmpmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>() -> IWMPControls_Vtbl {
        unsafe extern "system" fn isAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn play<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).play().into()
        }
        unsafe extern "system" fn stop<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stop().into()
        }
        unsafe extern "system" fn pause<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pause().into()
        }
        unsafe extern "system" fn fastForward<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fastForward().into()
        }
        unsafe extern "system" fn fastReverse<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fastReverse().into()
        }
        unsafe extern "system" fn currentPosition<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentPosition(::core::mem::transmute_copy(&pdcurrentposition)).into()
        }
        unsafe extern "system" fn SetcurrentPosition<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentPosition(::core::mem::transmute_copy(&dcurrentposition)).into()
        }
        unsafe extern "system" fn currentPositionString<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentPositionString(::core::mem::transmute_copy(&pbstrcurrentposition)).into()
        }
        unsafe extern "system" fn next<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).next().into()
        }
        unsafe extern "system" fn previous<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).previous().into()
        }
        unsafe extern "system" fn currentItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).currentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn currentMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentMarker(::core::mem::transmute_copy(&plmarker)).into()
        }
        unsafe extern "system" fn SetcurrentMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentMarker(::core::mem::transmute_copy(&lmarker)).into()
        }
        unsafe extern "system" fn playItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).playItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAvailable: isAvailable::<Identity, Impl, OFFSET>,
            play: play::<Identity, Impl, OFFSET>,
            stop: stop::<Identity, Impl, OFFSET>,
            pause: pause::<Identity, Impl, OFFSET>,
            fastForward: fastForward::<Identity, Impl, OFFSET>,
            fastReverse: fastReverse::<Identity, Impl, OFFSET>,
            currentPosition: currentPosition::<Identity, Impl, OFFSET>,
            SetcurrentPosition: SetcurrentPosition::<Identity, Impl, OFFSET>,
            currentPositionString: currentPositionString::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            previous: previous::<Identity, Impl, OFFSET>,
            currentItem: currentItem::<Identity, Impl, OFFSET>,
            SetcurrentItem: SetcurrentItem::<Identity, Impl, OFFSET>,
            currentMarker: currentMarker::<Identity, Impl, OFFSET>,
            SetcurrentMarker: SetcurrentMarker::<Identity, Impl, OFFSET>,
            playItem: playItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPControls_Impl {
    fn step(&mut self, lstep: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls2_Impl, const OFFSET: isize>() -> IWMPControls2_Vtbl {
        unsafe extern "system" fn step<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).step(::core::mem::transmute_copy(&lstep)).into()
        }
        Self { base: IWMPControls_Vtbl::new::<Identity, Impl, OFFSET>(), step: step::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPControls as ::windows::core::Interface>::IID
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
    fn SetcurrentPositionTimecode(&mut self, bstrtimecode: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>() -> IWMPControls3_Vtbl {
        unsafe extern "system" fn audioLanguageCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).audioLanguageCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAudioLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAudioLanguageID(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn getAudioLanguageDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAudioLanguageDescription(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrlangdesc)).into()
        }
        unsafe extern "system" fn currentAudioLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentAudioLanguage(::core::mem::transmute_copy(&llangid)).into()
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentAudioLanguageIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentAudioLanguageIndex(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn getLanguageName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getLanguageName(::core::mem::transmute_copy(&llangid), ::core::mem::transmute_copy(&pbstrlangname)).into()
        }
        unsafe extern "system" fn currentPositionTimecode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentPositionTimecode(::core::mem::transmute_copy(&bstrtimecode)).into()
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentPositionTimecode(::core::mem::transmute_copy(&bstrtimecode)).into()
        }
        Self {
            base: IWMPControls2_Vtbl::new::<Identity, Impl, OFFSET>(),
            audioLanguageCount: audioLanguageCount::<Identity, Impl, OFFSET>,
            getAudioLanguageID: getAudioLanguageID::<Identity, Impl, OFFSET>,
            getAudioLanguageDescription: getAudioLanguageDescription::<Identity, Impl, OFFSET>,
            currentAudioLanguage: currentAudioLanguage::<Identity, Impl, OFFSET>,
            SetcurrentAudioLanguage: SetcurrentAudioLanguage::<Identity, Impl, OFFSET>,
            currentAudioLanguageIndex: currentAudioLanguageIndex::<Identity, Impl, OFFSET>,
            SetcurrentAudioLanguageIndex: SetcurrentAudioLanguageIndex::<Identity, Impl, OFFSET>,
            getLanguageName: getLanguageName::<Identity, Impl, OFFSET>,
            currentPositionTimecode: currentPositionTimecode::<Identity, Impl, OFFSET>,
            SetcurrentPositionTimecode: SetcurrentPositionTimecode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPControls as ::windows::core::Interface>::IID || iid == &<IWMPControls2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPConvert_Impl: Sized {
    fn ConvertFile(&mut self, bstrinputfile: &super::super::Foundation::BSTR, bstrdestinationfolder: &super::super::Foundation::BSTR, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetErrorURL(&mut self, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPConvert_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvert_Impl, const OFFSET: isize>() -> IWMPConvert_Vtbl {
        unsafe extern "system" fn ConvertFile<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutputfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertFile(::core::mem::transmute_copy(&bstrinputfile), ::core::mem::transmute_copy(&bstrdestinationfolder), ::core::mem::transmute_copy(&pbstroutputfile)).into()
        }
        unsafe extern "system" fn GetErrorURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrorURL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConvertFile: ConvertFile::<Identity, Impl, OFFSET>,
            GetErrorURL: GetErrorURL::<Identity, Impl, OFFSET>,
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
    fn SetURL(&mut self, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn openState(&mut self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()>;
    fn playState(&mut self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()>;
    fn controls(&mut self) -> ::windows::core::Result<IWMPControls>;
    fn settings(&mut self) -> ::windows::core::Result<IWMPSettings>;
    fn currentMedia(&mut self) -> ::windows::core::Result<IWMPMedia>;
    fn SetcurrentMedia(&mut self, pmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn mediaCollection(&mut self) -> ::windows::core::Result<IWMPMediaCollection>;
    fn playlistCollection(&mut self) -> ::windows::core::Result<IWMPPlaylistCollection>;
    fn versionInfo(&mut self, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn launchURL(&mut self, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn network(&mut self) -> ::windows::core::Result<IWMPNetwork>;
    fn currentPlaylist(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetcurrentPlaylist(&mut self, ppl: &::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cdromCollection(&mut self) -> ::windows::core::Result<IWMPCdromCollection>;
    fn closedCaption(&mut self) -> ::windows::core::Result<IWMPClosedCaption>;
    fn isOnline(&mut self, pfonline: *mut i16) -> ::windows::core::Result<()>;
    fn error(&mut self) -> ::windows::core::Result<IWMPError>;
    fn status(&mut self, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>() -> IWMPCore_Vtbl {
        unsafe extern "system" fn close<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).close().into()
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURL(::core::mem::transmute_copy(&bstrurl)).into()
        }
        unsafe extern "system" fn openState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).openState(::core::mem::transmute_copy(&pwmpos)).into()
        }
        unsafe extern "system" fn playState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).playState(::core::mem::transmute_copy(&pwmpps)).into()
        }
        unsafe extern "system" fn controls<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).controls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn settings<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).settings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMedia<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).currentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMedia<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentMedia(::core::mem::transmute(&pmedia)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediacollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlistCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).playlistCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn versionInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).versionInfo(::core::mem::transmute_copy(&pbstrversioninfo)).into()
        }
        unsafe extern "system" fn launchURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).launchURL(::core::mem::transmute_copy(&bstrurl)).into()
        }
        unsafe extern "system" fn network<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqni: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).network() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqni = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).currentPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    *pppl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetcurrentPlaylist(::core::mem::transmute(&ppl)).into()
        }
        unsafe extern "system" fn cdromCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).cdromCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcdromcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn closedCaption<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).closedCaption() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclosedcaption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOnline<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfonline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isOnline(::core::mem::transmute_copy(&pfonline)).into()
        }
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).error() {
                ::core::result::Result::Ok(ok__) => {
                    *pperror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).status(::core::mem::transmute_copy(&pbstrstatus)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            close: close::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            openState: openState::<Identity, Impl, OFFSET>,
            playState: playState::<Identity, Impl, OFFSET>,
            controls: controls::<Identity, Impl, OFFSET>,
            settings: settings::<Identity, Impl, OFFSET>,
            currentMedia: currentMedia::<Identity, Impl, OFFSET>,
            SetcurrentMedia: SetcurrentMedia::<Identity, Impl, OFFSET>,
            mediaCollection: mediaCollection::<Identity, Impl, OFFSET>,
            playlistCollection: playlistCollection::<Identity, Impl, OFFSET>,
            versionInfo: versionInfo::<Identity, Impl, OFFSET>,
            launchURL: launchURL::<Identity, Impl, OFFSET>,
            network: network::<Identity, Impl, OFFSET>,
            currentPlaylist: currentPlaylist::<Identity, Impl, OFFSET>,
            SetcurrentPlaylist: SetcurrentPlaylist::<Identity, Impl, OFFSET>,
            cdromCollection: cdromCollection::<Identity, Impl, OFFSET>,
            closedCaption: closedCaption::<Identity, Impl, OFFSET>,
            isOnline: isOnline::<Identity, Impl, OFFSET>,
            error: error::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl {
    fn dvd(&mut self) -> ::windows::core::Result<IWMPDVD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore2_Impl, const OFFSET: isize>() -> IWMPCore2_Vtbl {
        unsafe extern "system" fn dvd<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvd: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).dvd() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdvd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(), dvd: dvd::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPCore_Impl + IWMPCore2_Impl {
    fn newPlaylist(&mut self, bstrname: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn newMedia(&mut self, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPMedia>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3_Impl, const OFFSET: isize>() -> IWMPCore3_Vtbl {
        unsafe extern "system" fn newPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).newPlaylist(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn newMedia<Identity: ::windows::core::IUnknownImpl, Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).newMedia(::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMPCore2_Vtbl::new::<Identity, Impl, OFFSET>(),
            newPlaylist: newPlaylist::<Identity, Impl, OFFSET>,
            newMedia: newMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID || iid == &<IWMPCore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDVD_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: &super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn domain(&mut self, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn topMenu(&mut self) -> ::windows::core::Result<()>;
    fn titleMenu(&mut self) -> ::windows::core::Result<()>;
    fn back(&mut self) -> ::windows::core::Result<()>;
    fn resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDVD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>() -> IWMPDVD_Vtbl {
        unsafe extern "system" fn isAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn domain<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).domain(::core::mem::transmute_copy(&strdomain)).into()
        }
        unsafe extern "system" fn topMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).topMenu().into()
        }
        unsafe extern "system" fn titleMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).titleMenu().into()
        }
        unsafe extern "system" fn back<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).back().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).resume().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAvailable: isAvailable::<Identity, Impl, OFFSET>,
            domain: domain::<Identity, Impl, OFFSET>,
            topMenu: topMenu::<Identity, Impl, OFFSET>,
            titleMenu: titleMenu::<Identity, Impl, OFFSET>,
            back: back::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDVD as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn id(&mut self, plid: *mut i32) -> ::windows::core::Result<()>;
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, litem: i32) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn startDownload(&mut self, bstrsourceurl: &super::super::Foundation::BSTR, bstrtype: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn removeItem(&mut self, litem: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>() -> IWMPDownloadCollection_Vtbl {
        unsafe extern "system" fn id<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).id(::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&litem)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDownload<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).startDownload(::core::mem::transmute_copy(&bstrsourceurl), ::core::mem::transmute_copy(&bstrtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdownload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeItem(::core::mem::transmute_copy(&litem)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            id: id::<Identity, Impl, OFFSET>,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            startDownload: startDownload::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>() -> IWMPDownloadItem_Vtbl {
        unsafe extern "system" fn sourceURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).sourceURL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn size<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).size(::core::mem::transmute_copy(&plsize)).into()
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).r#type(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn progress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn downloadState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).downloadState(::core::mem::transmute_copy(&pwmpsdls)).into()
        }
        unsafe extern "system" fn pause<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pause().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).resume().into()
        }
        unsafe extern "system" fn cancel<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).cancel().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            sourceURL: sourceURL::<Identity, Impl, OFFSET>,
            size: size::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            progress: progress::<Identity, Impl, OFFSET>,
            downloadState: downloadState::<Identity, Impl, OFFSET>,
            pause: pause::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            cancel: cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPDownloadItem_Impl {
    fn getItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem2_Impl, const OFFSET: isize>() -> IWMPDownloadItem2_Vtbl {
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base: IWMPDownloadItem_Vtbl::new::<Identity, Impl, OFFSET>(), getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPDownloadItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn getDownloadCollection(&mut self, lcollectionid: i32) -> ::windows::core::Result<IWMPDownloadCollection>;
    fn createDownloadCollection(&mut self) -> ::windows::core::Result<IWMPDownloadCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>() -> IWMPDownloadManager_Vtbl {
        unsafe extern "system" fn getDownloadCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDownloadCollection(::core::mem::transmute_copy(&lcollectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDownloadCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createDownloadCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getDownloadCollection: getDownloadCollection::<Identity, Impl, OFFSET>,
            createDownloadCollection: createDownloadCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IWMPEffects_Impl: Sized {
    fn Render(&mut self, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn MediaInfo(&mut self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>() -> IWMPEffects_Vtbl {
        unsafe extern "system" fn Render<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&plevels), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn MediaInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaInfo(::core::mem::transmute_copy(&lchannelcount), ::core::mem::transmute_copy(&lsamplerate), ::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilities)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetPresetTitle<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPresetTitle(::core::mem::transmute_copy(&npreset), ::core::mem::transmute_copy(&bstrpresettitle)).into()
        }
        unsafe extern "system" fn GetPresetCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPresetCount(::core::mem::transmute_copy(&pnpresetcount)).into()
        }
        unsafe extern "system" fn SetCurrentPreset<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentPreset(::core::mem::transmute_copy(&npreset)).into()
        }
        unsafe extern "system" fn GetCurrentPreset<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentPreset(::core::mem::transmute_copy(&pnpreset)).into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayPropertyPage(::core::mem::transmute_copy(&hwndowner)).into()
        }
        unsafe extern "system" fn GoFullscreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GoFullscreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn RenderFullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenderFullScreen(::core::mem::transmute_copy(&plevels)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Render: Render::<Identity, Impl, OFFSET>,
            MediaInfo: MediaInfo::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            GetPresetTitle: GetPresetTitle::<Identity, Impl, OFFSET>,
            GetPresetCount: GetPresetCount::<Identity, Impl, OFFSET>,
            SetCurrentPreset: SetCurrentPreset::<Identity, Impl, OFFSET>,
            GetCurrentPreset: GetCurrentPreset::<Identity, Impl, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, Impl, OFFSET>,
            GoFullscreen: GoFullscreen::<Identity, Impl, OFFSET>,
            RenderFullScreen: RenderFullScreen::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IWMPEffects2_Impl: Sized + IWMPEffects_Impl {
    fn SetCore(&mut self, pplayer: &::core::option::Option<IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn NotifyNewMedia(&mut self, pmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn OnWindowMessage(&mut self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn RenderWindowed(&mut self, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IWMPEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>() -> IWMPEffects2_Vtbl {
        unsafe extern "system" fn SetCore<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCore(::core::mem::transmute(&pplayer)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn NotifyNewMedia<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyNewMedia(::core::mem::transmute(&pmedia)).into()
        }
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresultparam)).into()
        }
        unsafe extern "system" fn RenderWindowed<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenderWindowed(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&frequiredrender)).into()
        }
        Self {
            base: IWMPEffects_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetCore: SetCore::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            NotifyNewMedia: NotifyNewMedia::<Identity, Impl, OFFSET>,
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            RenderWindowed: RenderWindowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEffects2 as ::windows::core::Interface>::IID || iid == &<IWMPEffects as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const OFFSET: isize>() -> IWMPError_Vtbl {
        unsafe extern "system" fn clearErrorQueue<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).clearErrorQueue().into()
        }
        unsafe extern "system" fn errorCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).errorCount(::core::mem::transmute_copy(&plnumerrors)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pperroritem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn webHelp<Identity: ::windows::core::IUnknownImpl, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).webHelp().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            clearErrorQueue: clearErrorQueue::<Identity, Impl, OFFSET>,
            errorCount: errorCount::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            webHelp: webHelp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPError as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>() -> IWMPErrorItem_Vtbl {
        unsafe extern "system" fn errorCode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).errorCode(::core::mem::transmute_copy(&phr)).into()
        }
        unsafe extern "system" fn errorDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).errorDescription(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn errorContext<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).errorContext(::core::mem::transmute_copy(&pvarcontext)).into()
        }
        unsafe extern "system" fn remedy<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).remedy(::core::mem::transmute_copy(&plremedy)).into()
        }
        unsafe extern "system" fn customUrl<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).customUrl(::core::mem::transmute_copy(&pbstrcustomurl)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            errorDescription: errorDescription::<Identity, Impl, OFFSET>,
            errorContext: errorContext::<Identity, Impl, OFFSET>,
            remedy: remedy::<Identity, Impl, OFFSET>,
            customUrl: customUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPErrorItem_Impl {
    fn condition(&mut self, plcondition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem2_Impl, const OFFSET: isize>() -> IWMPErrorItem2_Vtbl {
        unsafe extern "system" fn condition<Identity: ::windows::core::IUnknownImpl, Impl: IWMPErrorItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).condition(::core::mem::transmute_copy(&plcondition)).into()
        }
        Self { base: IWMPErrorItem_Vtbl::new::<Identity, Impl, OFFSET>(), condition: condition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPErrorItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents_Impl: Sized {
    fn OpenStateChange(&mut self, newstate: i32);
    fn PlayStateChange(&mut self, newstate: i32);
    fn AudioLanguageChange(&mut self, langid: i32);
    fn StatusChange(&mut self);
    fn ScriptCommand(&mut self, sctype: &super::super::Foundation::BSTR, param: &super::super::Foundation::BSTR);
    fn NewStream(&mut self);
    fn Disconnect(&mut self, result: i32);
    fn Buffering(&mut self, start: i16);
    fn Error(&mut self);
    fn Warning(&mut self, warningtype: i32, param: i32, description: &super::super::Foundation::BSTR);
    fn EndOfStream(&mut self, result: i32);
    fn PositionChange(&mut self, oldposition: f64, newposition: f64);
    fn MarkerHit(&mut self, markernum: i32);
    fn DurationUnitChange(&mut self, newdurationunit: i32);
    fn CdromMediaChange(&mut self, cdromnum: i32);
    fn PlaylistChange(&mut self, playlist: &::core::option::Option<super::super::System::Com::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(&mut self, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(&mut self, bstritemname: &super::super::Foundation::BSTR);
    fn MediaChange(&mut self, item: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn CurrentMediaItemAvailable(&mut self, bstritemname: &super::super::Foundation::BSTR);
    fn CurrentItemChange(&mut self, pdispmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn MediaCollectionChange(&mut self);
    fn MediaCollectionAttributeStringAdded(&mut self, bstrattribname: &super::super::Foundation::BSTR, bstrattribval: &super::super::Foundation::BSTR);
    fn MediaCollectionAttributeStringRemoved(&mut self, bstrattribname: &super::super::Foundation::BSTR, bstrattribval: &super::super::Foundation::BSTR);
    fn MediaCollectionAttributeStringChanged(&mut self, bstrattribname: &super::super::Foundation::BSTR, bstroldattribval: &super::super::Foundation::BSTR, bstrnewattribval: &super::super::Foundation::BSTR);
    fn PlaylistCollectionChange(&mut self);
    fn PlaylistCollectionPlaylistAdded(&mut self, bstrplaylistname: &super::super::Foundation::BSTR);
    fn PlaylistCollectionPlaylistRemoved(&mut self, bstrplaylistname: &super::super::Foundation::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(&mut self, bstrplaylistname: &super::super::Foundation::BSTR, varfisdeleted: i16);
    fn ModeChange(&mut self, modename: &super::super::Foundation::BSTR, newvalue: i16);
    fn MediaError(&mut self, pmediaobject: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn OpenPlaylistSwitch(&mut self, pitem: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn DomainChange(&mut self, strdomain: &super::super::Foundation::BSTR);
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>() -> IWMPEvents_Vtbl {
        unsafe extern "system" fn OpenStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn PlayStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn AudioLanguageChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AudioLanguageChange(::core::mem::transmute_copy(&langid))
        }
        unsafe extern "system" fn StatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StatusChange()
        }
        unsafe extern "system" fn ScriptCommand<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sctype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, param: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScriptCommand(::core::mem::transmute_copy(&sctype), ::core::mem::transmute_copy(&param))
        }
        unsafe extern "system" fn NewStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NewStream()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn Buffering<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Buffering(::core::mem::transmute_copy(&start))
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error()
        }
        unsafe extern "system" fn Warning<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Warning(::core::mem::transmute_copy(&warningtype), ::core::mem::transmute_copy(&param), ::core::mem::transmute_copy(&description))
        }
        unsafe extern "system" fn EndOfStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndOfStream(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn PositionChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PositionChange(::core::mem::transmute_copy(&oldposition), ::core::mem::transmute_copy(&newposition))
        }
        unsafe extern "system" fn MarkerHit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MarkerHit(::core::mem::transmute_copy(&markernum))
        }
        unsafe extern "system" fn DurationUnitChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DurationUnitChange(::core::mem::transmute_copy(&newdurationunit))
        }
        unsafe extern "system" fn CdromMediaChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromMediaChange(::core::mem::transmute_copy(&cdromnum))
        }
        unsafe extern "system" fn PlaylistChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlist: ::windows::core::RawPtr, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlaylistChange(::core::mem::transmute(&playlist), ::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CurrentPlaylistChange(::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CurrentPlaylistItemAvailable(::core::mem::transmute_copy(&bstritemname))
        }
        unsafe extern "system" fn MediaChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaChange(::core::mem::transmute(&item))
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CurrentMediaItemAvailable(::core::mem::transmute_copy(&bstritemname))
        }
        unsafe extern "system" fn CurrentItemChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CurrentItemChange(::core::mem::transmute(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionChange()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionAttributeStringAdded(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionAttributeStringRemoved(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroldattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewattribval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionAttributeStringChanged(::core::mem::transmute_copy(&bstrattribname), ::core::mem::transmute_copy(&bstroldattribval), ::core::mem::transmute_copy(&bstrnewattribval))
        }
        unsafe extern "system" fn PlaylistCollectionChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlaylistCollectionChange()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlaylistCollectionPlaylistAdded(::core::mem::transmute_copy(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlaylistCollectionPlaylistRemoved(::core::mem::transmute_copy(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfisdeleted: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlaylistCollectionPlaylistSetAsDeleted(::core::mem::transmute_copy(&bstrplaylistname), ::core::mem::transmute_copy(&varfisdeleted))
        }
        unsafe extern "system" fn ModeChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newvalue: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModeChange(::core::mem::transmute_copy(&modename), ::core::mem::transmute_copy(&newvalue))
        }
        unsafe extern "system" fn MediaError<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediaobject: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaError(::core::mem::transmute(&pmediaobject))
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenPlaylistSwitch(::core::mem::transmute(&pitem))
        }
        unsafe extern "system" fn DomainChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DomainChange(::core::mem::transmute_copy(&strdomain))
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwitchedToPlayerApplication()
        }
        unsafe extern "system" fn SwitchedToControl<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwitchedToControl()
        }
        unsafe extern "system" fn PlayerDockedStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayerDockedStateChange()
        }
        unsafe extern "system" fn PlayerReconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PlayerReconnect()
        }
        unsafe extern "system" fn Click<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Click(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn DoubleClick<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DoubleClick(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeyDown(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn KeyPress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeyPress(::core::mem::transmute_copy(&nkeyascii))
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeyUp(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn MouseDown<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MouseDown(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseMove<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MouseMove(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseUp<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MouseUp(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenStateChange: OpenStateChange::<Identity, Impl, OFFSET>,
            PlayStateChange: PlayStateChange::<Identity, Impl, OFFSET>,
            AudioLanguageChange: AudioLanguageChange::<Identity, Impl, OFFSET>,
            StatusChange: StatusChange::<Identity, Impl, OFFSET>,
            ScriptCommand: ScriptCommand::<Identity, Impl, OFFSET>,
            NewStream: NewStream::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Buffering: Buffering::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            Warning: Warning::<Identity, Impl, OFFSET>,
            EndOfStream: EndOfStream::<Identity, Impl, OFFSET>,
            PositionChange: PositionChange::<Identity, Impl, OFFSET>,
            MarkerHit: MarkerHit::<Identity, Impl, OFFSET>,
            DurationUnitChange: DurationUnitChange::<Identity, Impl, OFFSET>,
            CdromMediaChange: CdromMediaChange::<Identity, Impl, OFFSET>,
            PlaylistChange: PlaylistChange::<Identity, Impl, OFFSET>,
            CurrentPlaylistChange: CurrentPlaylistChange::<Identity, Impl, OFFSET>,
            CurrentPlaylistItemAvailable: CurrentPlaylistItemAvailable::<Identity, Impl, OFFSET>,
            MediaChange: MediaChange::<Identity, Impl, OFFSET>,
            CurrentMediaItemAvailable: CurrentMediaItemAvailable::<Identity, Impl, OFFSET>,
            CurrentItemChange: CurrentItemChange::<Identity, Impl, OFFSET>,
            MediaCollectionChange: MediaCollectionChange::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringAdded: MediaCollectionAttributeStringAdded::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringRemoved: MediaCollectionAttributeStringRemoved::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringChanged: MediaCollectionAttributeStringChanged::<Identity, Impl, OFFSET>,
            PlaylistCollectionChange: PlaylistCollectionChange::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistAdded: PlaylistCollectionPlaylistAdded::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistRemoved: PlaylistCollectionPlaylistRemoved::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted: PlaylistCollectionPlaylistSetAsDeleted::<Identity, Impl, OFFSET>,
            ModeChange: ModeChange::<Identity, Impl, OFFSET>,
            MediaError: MediaError::<Identity, Impl, OFFSET>,
            OpenPlaylistSwitch: OpenPlaylistSwitch::<Identity, Impl, OFFSET>,
            DomainChange: DomainChange::<Identity, Impl, OFFSET>,
            SwitchedToPlayerApplication: SwitchedToPlayerApplication::<Identity, Impl, OFFSET>,
            SwitchedToControl: SwitchedToControl::<Identity, Impl, OFFSET>,
            PlayerDockedStateChange: PlayerDockedStateChange::<Identity, Impl, OFFSET>,
            PlayerReconnect: PlayerReconnect::<Identity, Impl, OFFSET>,
            Click: Click::<Identity, Impl, OFFSET>,
            DoubleClick: DoubleClick::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            KeyPress: KeyPress::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            MouseDown: MouseDown::<Identity, Impl, OFFSET>,
            MouseMove: MouseMove::<Identity, Impl, OFFSET>,
            MouseUp: MouseUp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents2_Impl: Sized + IWMPEvents_Impl {
    fn DeviceConnect(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>);
    fn DeviceDisconnect(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>);
    fn DeviceStatusChange(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, pmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn CreatePartnershipComplete(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, hrresult: ::windows::core::HRESULT);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>() -> IWMPEvents2_Vtbl {
        unsafe extern "system" fn DeviceConnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceConnect(::core::mem::transmute(&pdevice))
        }
        unsafe extern "system" fn DeviceDisconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceDisconnect(::core::mem::transmute(&pdevice))
        }
        unsafe extern "system" fn DeviceStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstatus: WMPDeviceStatus) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceStatusChange(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&newstatus))
        }
        unsafe extern "system" fn DeviceSyncStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, newstate: WMPSyncState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSyncStateChange(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn DeviceSyncError<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSyncError(::core::mem::transmute(&pdevice), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CreatePartnershipComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreatePartnershipComplete(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hrresult))
        }
        Self {
            base: IWMPEvents_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeviceConnect: DeviceConnect::<Identity, Impl, OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Identity, Impl, OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Identity, Impl, OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Identity, Impl, OFFSET>,
            DeviceSyncError: DeviceSyncError::<Identity, Impl, OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents2 as ::windows::core::Interface>::IID || iid == &<IWMPEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents3_Impl: Sized + IWMPEvents_Impl + IWMPEvents2_Impl {
    fn CdromRipStateChange(&mut self, pcdromrip: &::core::option::Option<IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(&mut self, pcdromrip: &::core::option::Option<IWMPCdromRip>, pmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn CdromBurnStateChange(&mut self, pcdromburn: &::core::option::Option<IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(&mut self, pcdromburn: &::core::option::Option<IWMPCdromBurn>, pmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn CdromBurnError(&mut self, pcdromburn: &::core::option::Option<IWMPCdromBurn>, hrerror: ::windows::core::HRESULT);
    fn LibraryConnect(&mut self, plibrary: &::core::option::Option<IWMPLibrary>);
    fn LibraryDisconnect(&mut self, plibrary: &::core::option::Option<IWMPLibrary>);
    fn FolderScanStateChange(&mut self, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(&mut self, pdispstringcollection: &::core::option::Option<super::super::System::Com::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(&mut self, pdispmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
    fn MediaCollectionMediaRemoved(&mut self, pdispmedia: &::core::option::Option<super::super::System::Com::IDispatch>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>() -> IWMPEvents3_Vtbl {
        unsafe extern "system" fn CdromRipStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, wmprs: WMPRipState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromRipStateChange(::core::mem::transmute(&pcdromrip), ::core::mem::transmute_copy(&wmprs))
        }
        unsafe extern "system" fn CdromRipMediaError<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromRipMediaError(::core::mem::transmute(&pcdromrip), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CdromBurnStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, wmpbs: WMPBurnState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromBurnStateChange(::core::mem::transmute(&pcdromburn), ::core::mem::transmute_copy(&wmpbs))
        }
        unsafe extern "system" fn CdromBurnMediaError<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, pmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromBurnMediaError(::core::mem::transmute(&pcdromburn), ::core::mem::transmute(&pmedia))
        }
        unsafe extern "system" fn CdromBurnError<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CdromBurnError(::core::mem::transmute(&pcdromburn), ::core::mem::transmute_copy(&hrerror))
        }
        unsafe extern "system" fn LibraryConnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LibraryConnect(::core::mem::transmute(&plibrary))
        }
        unsafe extern "system" fn LibraryDisconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LibraryDisconnect(::core::mem::transmute(&plibrary))
        }
        unsafe extern "system" fn FolderScanStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderScanStateChange(::core::mem::transmute_copy(&wmpfss))
        }
        unsafe extern "system" fn StringCollectionChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispstringcollection: ::windows::core::RawPtr, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StringCollectionChange(::core::mem::transmute(&pdispstringcollection), ::core::mem::transmute_copy(&change), ::core::mem::transmute_copy(&lcollectionindex))
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionMediaAdded(::core::mem::transmute(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MediaCollectionMediaRemoved(::core::mem::transmute(&pdispmedia))
        }
        Self {
            base: IWMPEvents2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CdromRipStateChange: CdromRipStateChange::<Identity, Impl, OFFSET>,
            CdromRipMediaError: CdromRipMediaError::<Identity, Impl, OFFSET>,
            CdromBurnStateChange: CdromBurnStateChange::<Identity, Impl, OFFSET>,
            CdromBurnMediaError: CdromBurnMediaError::<Identity, Impl, OFFSET>,
            CdromBurnError: CdromBurnError::<Identity, Impl, OFFSET>,
            LibraryConnect: LibraryConnect::<Identity, Impl, OFFSET>,
            LibraryDisconnect: LibraryDisconnect::<Identity, Impl, OFFSET>,
            FolderScanStateChange: FolderScanStateChange::<Identity, Impl, OFFSET>,
            StringCollectionChange: StringCollectionChange::<Identity, Impl, OFFSET>,
            MediaCollectionMediaAdded: MediaCollectionMediaAdded::<Identity, Impl, OFFSET>,
            MediaCollectionMediaRemoved: MediaCollectionMediaRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents3 as ::windows::core::Interface>::IID || iid == &<IWMPEvents as ::windows::core::Interface>::IID || iid == &<IWMPEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents4_Impl: Sized + IWMPEvents_Impl + IWMPEvents2_Impl + IWMPEvents3_Impl {
    fn DeviceEstimation(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents4_Impl, const OFFSET: isize>() -> IWMPEvents4_Vtbl {
        unsafe extern "system" fn DeviceEstimation<Identity: ::windows::core::IUnknownImpl, Impl: IWMPEvents4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceEstimation(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&qwestimatedusedspace), ::core::mem::transmute_copy(&qwestimatedspace))
        }
        Self { base: IWMPEvents3_Vtbl::new::<Identity, Impl, OFFSET>(), DeviceEstimation: DeviceEstimation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents4 as ::windows::core::Interface>::IID || iid == &<IWMPEvents as ::windows::core::Interface>::IID || iid == &<IWMPEvents2 as ::windows::core::Interface>::IID || iid == &<IWMPEvents3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPFolderMonitorServices_Impl: Sized {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn add(&mut self, bstrfolder: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>() -> IWMPFolderMonitorServices_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).add(::core::mem::transmute_copy(&bstrfolder)).into()
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn scanState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).scanState(::core::mem::transmute_copy(&pwmpfss)).into()
        }
        unsafe extern "system" fn currentFolder<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).currentFolder(::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn scannedFilesCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).scannedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn addedFilesCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn updateProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).updateProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startScan<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startScan().into()
        }
        unsafe extern "system" fn stopScan<Identity: ::windows::core::IUnknownImpl, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stopScan().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            add: add::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            scanState: scanState::<Identity, Impl, OFFSET>,
            currentFolder: currentFolder::<Identity, Impl, OFFSET>,
            scannedFilesCount: scannedFilesCount::<Identity, Impl, OFFSET>,
            addedFilesCount: addedFilesCount::<Identity, Impl, OFFSET>,
            updateProgress: updateProgress::<Identity, Impl, OFFSET>,
            startScan: startScan::<Identity, Impl, OFFSET>,
            stopScan: stopScan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPFolderMonitorServices as ::windows::core::Interface>::IID
    }
}
pub trait IWMPGraphCreation_Impl: Sized {
    fn GraphCreationPreRender(&mut self, pfiltergraph: &::core::option::Option<::windows::core::IUnknown>, preserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GraphCreationPostRender(&mut self, pfiltergraph: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetGraphCreationFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMPGraphCreation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>() -> IWMPGraphCreation_Vtbl {
        unsafe extern "system" fn GraphCreationPreRender<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GraphCreationPreRender(::core::mem::transmute(&pfiltergraph), ::core::mem::transmute(&preserved)).into()
        }
        unsafe extern "system" fn GraphCreationPostRender<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GraphCreationPostRender(::core::mem::transmute(&pfiltergraph)).into()
        }
        unsafe extern "system" fn GetGraphCreationFlags<Identity: ::windows::core::IUnknownImpl, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGraphCreationFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GraphCreationPreRender: GraphCreationPreRender::<Identity, Impl, OFFSET>,
            GraphCreationPostRender: GraphCreationPostRender::<Identity, Impl, OFFSET>,
            GetGraphCreationFlags: GetGraphCreationFlags::<Identity, Impl, OFFSET>,
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
    fn isIdentical(&mut self, piwmplibrary: &::core::option::Option<IWMPLibrary>, pvbool: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const OFFSET: isize>() -> IWMPLibrary_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).r#type(::core::mem::transmute_copy(&pwmplt)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmediacollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmplibrary: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmplibrary), ::core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            mediaCollection: mediaCollection::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary2_Impl: Sized + IWMPLibrary_Impl {
    fn getItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary2_Impl, const OFFSET: isize>() -> IWMPLibrary2_Vtbl {
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base: IWMPLibrary_Vtbl::new::<Identity, Impl, OFFSET>(), getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary2 as ::windows::core::Interface>::IID || iid == &<IWMPLibrary as ::windows::core::Interface>::IID
    }
}
pub trait IWMPLibraryServices_Impl: Sized {
    fn getCountByType(&mut self, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getLibraryByType(&mut self, wmplt: WMPLibraryType, lindex: i32) -> ::windows::core::Result<IWMPLibrary>;
}
impl IWMPLibraryServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>() -> IWMPLibraryServices_Vtbl {
        unsafe extern "system" fn getCountByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getCountByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getLibraryByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getLibraryByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmplibrary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getCountByType: getCountByType::<Identity, Impl, OFFSET>,
            getLibraryByType: getLibraryByType::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>() -> IWMPLibrarySharingServices_Vtbl {
        unsafe extern "system" fn isLibraryShared<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isLibraryShared(::core::mem::transmute_copy(&pvbshared)).into()
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isLibrarySharingEnabled(::core::mem::transmute_copy(&pvbenabled)).into()
        }
        unsafe extern "system" fn showLibrarySharing<Identity: ::windows::core::IUnknownImpl, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).showLibrarySharing().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            isLibraryShared: isLibraryShared::<Identity, Impl, OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Identity, Impl, OFFSET>,
            showLibrarySharing: showLibrarySharing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrarySharingServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isIdentical(&mut self, piwmpmedia: &::core::option::Option<IWMPMedia>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn sourceURL(&mut self, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn name(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn imageSourceWidth(&mut self, pwidth: *mut i32) -> ::windows::core::Result<()>;
    fn imageSourceHeight(&mut self, pheight: *mut i32) -> ::windows::core::Result<()>;
    fn markerCount(&mut self, pmarkercount: *mut i32) -> ::windows::core::Result<()>;
    fn getMarkerTime(&mut self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()>;
    fn getMarkerName(&mut self, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn duration(&mut self, pduration: *mut f64) -> ::windows::core::Result<()>;
    fn durationString(&mut self, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getAttributeName(&mut self, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfoByAtom(&mut self, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn isMemberOf(&mut self, pplaylist: &::core::option::Option<IWMPPlaylist>, pvarfismemberof: *mut i16) -> ::windows::core::Result<()>;
    fn isReadOnlyItem(&mut self, bstritemname: &super::super::Foundation::BSTR, pvarfisreadonly: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>() -> IWMPMedia_Vtbl {
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpmedia), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn sourceURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).sourceURL(::core::mem::transmute_copy(&pbstrsourceurl)).into()
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setname(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn imageSourceWidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).imageSourceWidth(::core::mem::transmute_copy(&pwidth)).into()
        }
        unsafe extern "system" fn imageSourceHeight<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).imageSourceHeight(::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn markerCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).markerCount(::core::mem::transmute_copy(&pmarkercount)).into()
        }
        unsafe extern "system" fn getMarkerTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getMarkerTime(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pmarkertime)).into()
        }
        unsafe extern "system" fn getMarkerName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getMarkerName(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pbstrmarkername)).into()
        }
        unsafe extern "system" fn duration<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn durationString<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).durationString(::core::mem::transmute_copy(&pbstrduration)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAttributeName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAttributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstritemname)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&bstrval)).into()
        }
        unsafe extern "system" fn getItemInfoByAtom<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfoByAtom(::core::mem::transmute_copy(&latom), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn isMemberOf<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: ::windows::core::RawPtr, pvarfismemberof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isMemberOf(::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pvarfismemberof)).into()
        }
        unsafe extern "system" fn isReadOnlyItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfisreadonly: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isReadOnlyItem(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pvarfisreadonly)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
            sourceURL: sourceURL::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Setname: Setname::<Identity, Impl, OFFSET>,
            imageSourceWidth: imageSourceWidth::<Identity, Impl, OFFSET>,
            imageSourceHeight: imageSourceHeight::<Identity, Impl, OFFSET>,
            markerCount: markerCount::<Identity, Impl, OFFSET>,
            getMarkerTime: getMarkerTime::<Identity, Impl, OFFSET>,
            getMarkerName: getMarkerName::<Identity, Impl, OFFSET>,
            duration: duration::<Identity, Impl, OFFSET>,
            durationString: durationString::<Identity, Impl, OFFSET>,
            attributeCount: attributeCount::<Identity, Impl, OFFSET>,
            getAttributeName: getAttributeName::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            setItemInfo: setItemInfo::<Identity, Impl, OFFSET>,
            getItemInfoByAtom: getItemInfoByAtom::<Identity, Impl, OFFSET>,
            isMemberOf: isMemberOf::<Identity, Impl, OFFSET>,
            isReadOnlyItem: isReadOnlyItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMedia_Impl {
    fn error(&mut self) -> ::windows::core::Result<IWMPErrorItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia2_Impl, const OFFSET: isize>() -> IWMPMedia2_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).error() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmperroritem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMPMedia_Vtbl::new::<Identity, Impl, OFFSET>(), error: error::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPMedia as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMedia_Impl + IWMPMedia2_Impl {
    fn getAttributeCountByType(&mut self, bstrtype: &super::super::Foundation::BSTR, bstrlanguage: &super::super::Foundation::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&mut self, bstrtype: &super::super::Foundation::BSTR, bstrlanguage: &super::super::Foundation::BSTR, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3_Impl, const OFFSET: isize>() -> IWMPMedia3_Vtbl {
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAttributeCountByType(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfoByType(::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base: IWMPMedia2_Vtbl::new::<Identity, Impl, OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPMedia as ::windows::core::Interface>::IID || iid == &<IWMPMedia2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn add(&mut self, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPMedia>;
    fn getAll(&mut self) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByGenre(&mut self, bstrgenre: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAuthor(&mut self, bstrauthor: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAlbum(&mut self, bstralbum: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAttribute(&mut self, bstrattribute: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn remove(&mut self, pitem: &::core::option::Option<IWMPMedia>, varfdeletefile: i16) -> ::windows::core::Result<()>;
    fn getAttributeStringCollection(&mut self, bstrattribute: &super::super::Foundation::BSTR, bstrmediatype: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPStringCollection>;
    fn getMediaAtom(&mut self, bstritemname: &super::super::Foundation::BSTR, platom: *mut i32) -> ::windows::core::Result<()>;
    fn setDeleted(&mut self, pitem: &::core::option::Option<IWMPMedia>, varfisdeleted: i16) -> ::windows::core::Result<()>;
    fn isDeleted(&mut self, pitem: &::core::option::Option<IWMPMedia>, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>() -> IWMPMediaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).add(::core::mem::transmute_copy(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByGenre<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgenre: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByGenre(::core::mem::transmute_copy(&bstrgenre)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAuthor<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByAuthor(::core::mem::transmute_copy(&bstrauthor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAlbum<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstralbum: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByAlbum(::core::mem::transmute_copy(&bstralbum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByAttribute(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfdeletefile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).remove(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfdeletefile)).into()
        }
        unsafe extern "system" fn getAttributeStringCollection<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAttributeStringCollection(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstringcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMediaAtom<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getMediaAtom(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&platom)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            add: add::<Identity, Impl, OFFSET>,
            getAll: getAll::<Identity, Impl, OFFSET>,
            getByName: getByName::<Identity, Impl, OFFSET>,
            getByGenre: getByGenre::<Identity, Impl, OFFSET>,
            getByAuthor: getByAuthor::<Identity, Impl, OFFSET>,
            getByAlbum: getByAlbum::<Identity, Impl, OFFSET>,
            getByAttribute: getByAttribute::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            getAttributeStringCollection: getAttributeStringCollection::<Identity, Impl, OFFSET>,
            getMediaAtom: getMediaAtom::<Identity, Impl, OFFSET>,
            setDeleted: setDeleted::<Identity, Impl, OFFSET>,
            isDeleted: isDeleted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPMediaCollection_Impl {
    fn createQuery(&mut self) -> ::windows::core::Result<IWMPQuery>;
    fn getPlaylistByQuery(&mut self, pquery: &::core::option::Option<IWMPQuery>, bstrmediatype: &super::super::Foundation::BSTR, bstrsortattribute: &super::super::Foundation::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(&mut self, bstrattribute: &super::super::Foundation::BSTR, pquery: &::core::option::Option<IWMPQuery>, bstrmediatype: &super::super::Foundation::BSTR, bstrsortattribute: &super::super::Foundation::BSTR, fsortascending: i16) -> ::windows::core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(&mut self, bstrattribute: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR, bstrmediatype: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>() -> IWMPMediaCollection2_Vtbl {
        unsafe extern "system" fn createQuery<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppplaylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getPlaylistByQuery(::core::mem::transmute(&pquery), ::core::mem::transmute_copy(&bstrmediatype), ::core::mem::transmute_copy(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pquery: ::windows::core::RawPtr, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsortattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fsortascending: i16, ppstringcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getStringCollectionByQuery(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute(&pquery), ::core::mem::transmute_copy(&bstrmediatype), ::core::mem::transmute_copy(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstringcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmediatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmediaitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByAttributeAndMediaType(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstrvalue), ::core::mem::transmute_copy(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMPMediaCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            createQuery: createQuery::<Identity, Impl, OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Identity, Impl, OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Identity, Impl, OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPMediaCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPMediaPluginRegistrar_Impl: Sized {
    fn WMPRegisterPlayerPlugin(&mut self, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: &::windows::core::GUID, clsid: &::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn WMPUnRegisterPlayerPlugin(&mut self, guidplugintype: &::windows::core::GUID, clsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPMediaPluginRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>() -> IWMPMediaPluginRegistrar_Vtbl {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, pwszuninstallstring: super::super::Foundation::PWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMPRegisterPlayerPlugin(::core::mem::transmute_copy(&pwszfriendlyname), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pwszuninstallstring), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute_copy(&guidplugintype), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&cmediatypes), ::core::mem::transmute_copy(&pmediatypes)).into()
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMPUnRegisterPlayerPlugin(::core::mem::transmute_copy(&guidplugintype), ::core::mem::transmute_copy(&clsid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WMPRegisterPlayerPlugin: WMPRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
            WMPUnRegisterPlayerPlugin: WMPUnRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>() -> IWMPMetadataPicture_Vtbl {
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).mimeType(::core::mem::transmute_copy(&pbstrmimetype)).into()
        }
        unsafe extern "system" fn pictureType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pictureType(::core::mem::transmute_copy(&pbstrpicturetype)).into()
        }
        unsafe extern "system" fn description<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            pictureType: pictureType::<Identity, Impl, OFFSET>,
            description: description::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataPicture as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMetadataText_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn description(&mut self, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn text(&mut self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataText_Impl, const OFFSET: isize>() -> IWMPMetadataText_Vtbl {
        unsafe extern "system" fn description<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl, Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).text(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            description: description::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataText as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    fn getProxySettings(&mut self, bstrprotocol: &super::super::Foundation::BSTR, plproxysetting: *mut i32) -> ::windows::core::Result<()>;
    fn setProxySettings(&mut self, bstrprotocol: &super::super::Foundation::BSTR, lproxysetting: i32) -> ::windows::core::Result<()>;
    fn getProxyName(&mut self, bstrprotocol: &super::super::Foundation::BSTR, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setProxyName(&mut self, bstrprotocol: &super::super::Foundation::BSTR, bstrproxyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getProxyPort(&mut self, bstrprotocol: &super::super::Foundation::BSTR, lproxyport: *mut i32) -> ::windows::core::Result<()>;
    fn setProxyPort(&mut self, bstrprotocol: &super::super::Foundation::BSTR, lproxyport: i32) -> ::windows::core::Result<()>;
    fn getProxyExceptionList(&mut self, bstrprotocol: &super::super::Foundation::BSTR, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setProxyExceptionList(&mut self, bstrprotocol: &super::super::Foundation::BSTR, pbstrexceptionlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getProxyBypassForLocal(&mut self, bstrprotocol: &super::super::Foundation::BSTR, pfbypassforlocal: *mut i16) -> ::windows::core::Result<()>;
    fn setProxyBypassForLocal(&mut self, bstrprotocol: &super::super::Foundation::BSTR, fbypassforlocal: i16) -> ::windows::core::Result<()>;
    fn maxBandwidth(&mut self, lmaxbandwidth: *mut i32) -> ::windows::core::Result<()>;
    fn SetmaxBandwidth(&mut self, lmaxbandwidth: i32) -> ::windows::core::Result<()>;
    fn downloadProgress(&mut self, pldownloadprogress: *mut i32) -> ::windows::core::Result<()>;
    fn encodedFrameRate(&mut self, plframerate: *mut i32) -> ::windows::core::Result<()>;
    fn framesSkipped(&mut self, plframes: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>() -> IWMPNetwork_Vtbl {
        unsafe extern "system" fn bandWidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).bandWidth(::core::mem::transmute_copy(&plbandwidth)).into()
        }
        unsafe extern "system" fn recoveredPackets<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).recoveredPackets(::core::mem::transmute_copy(&plrecoveredpackets)).into()
        }
        unsafe extern "system" fn sourceProtocol<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).sourceProtocol(::core::mem::transmute_copy(&pbstrsourceprotocol)).into()
        }
        unsafe extern "system" fn receivedPackets<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).receivedPackets(::core::mem::transmute_copy(&plreceivedpackets)).into()
        }
        unsafe extern "system" fn lostPackets<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).lostPackets(::core::mem::transmute_copy(&pllostpackets)).into()
        }
        unsafe extern "system" fn receptionQuality<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).receptionQuality(::core::mem::transmute_copy(&plreceptionquality)).into()
        }
        unsafe extern "system" fn bufferingCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).bufferingCount(::core::mem::transmute_copy(&plbufferingcount)).into()
        }
        unsafe extern "system" fn bufferingProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).bufferingProgress(::core::mem::transmute_copy(&plbufferingprogress)).into()
        }
        unsafe extern "system" fn bufferingTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).bufferingTime(::core::mem::transmute_copy(&plbufferingtime)).into()
        }
        unsafe extern "system" fn SetbufferingTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetbufferingTime(::core::mem::transmute_copy(&lbufferingtime)).into()
        }
        unsafe extern "system" fn frameRate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).frameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn maxBitRate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).maxBitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn bitRate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).bitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn getProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getProxySettings(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&plproxysetting)).into()
        }
        unsafe extern "system" fn setProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxySettings(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxysetting)).into()
        }
        unsafe extern "system" fn getProxyName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproxyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getProxyName(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrproxyname)).into()
        }
        unsafe extern "system" fn setProxyName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxyName(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrproxyname)).into()
        }
        unsafe extern "system" fn getProxyPort<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getProxyPort(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn setProxyPort<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxyPort(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn getProxyExceptionList<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getProxyExceptionList(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn setProxyExceptionList<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrexceptionlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxyExceptionList(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn getProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfbypassforlocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getProxyBypassForLocal(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&pfbypassforlocal)).into()
        }
        unsafe extern "system" fn setProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fbypassforlocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxyBypassForLocal(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn maxBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).maxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn SetmaxBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetmaxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn downloadProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).downloadProgress(::core::mem::transmute_copy(&pldownloadprogress)).into()
        }
        unsafe extern "system" fn encodedFrameRate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).encodedFrameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn framesSkipped<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).framesSkipped(::core::mem::transmute_copy(&plframes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            bandWidth: bandWidth::<Identity, Impl, OFFSET>,
            recoveredPackets: recoveredPackets::<Identity, Impl, OFFSET>,
            sourceProtocol: sourceProtocol::<Identity, Impl, OFFSET>,
            receivedPackets: receivedPackets::<Identity, Impl, OFFSET>,
            lostPackets: lostPackets::<Identity, Impl, OFFSET>,
            receptionQuality: receptionQuality::<Identity, Impl, OFFSET>,
            bufferingCount: bufferingCount::<Identity, Impl, OFFSET>,
            bufferingProgress: bufferingProgress::<Identity, Impl, OFFSET>,
            bufferingTime: bufferingTime::<Identity, Impl, OFFSET>,
            SetbufferingTime: SetbufferingTime::<Identity, Impl, OFFSET>,
            frameRate: frameRate::<Identity, Impl, OFFSET>,
            maxBitRate: maxBitRate::<Identity, Impl, OFFSET>,
            bitRate: bitRate::<Identity, Impl, OFFSET>,
            getProxySettings: getProxySettings::<Identity, Impl, OFFSET>,
            setProxySettings: setProxySettings::<Identity, Impl, OFFSET>,
            getProxyName: getProxyName::<Identity, Impl, OFFSET>,
            setProxyName: setProxyName::<Identity, Impl, OFFSET>,
            getProxyPort: getProxyPort::<Identity, Impl, OFFSET>,
            setProxyPort: setProxyPort::<Identity, Impl, OFFSET>,
            getProxyExceptionList: getProxyExceptionList::<Identity, Impl, OFFSET>,
            setProxyExceptionList: setProxyExceptionList::<Identity, Impl, OFFSET>,
            getProxyBypassForLocal: getProxyBypassForLocal::<Identity, Impl, OFFSET>,
            setProxyBypassForLocal: setProxyBypassForLocal::<Identity, Impl, OFFSET>,
            maxBandwidth: maxBandwidth::<Identity, Impl, OFFSET>,
            SetmaxBandwidth: SetmaxBandwidth::<Identity, Impl, OFFSET>,
            downloadProgress: downloadProgress::<Identity, Impl, OFFSET>,
            encodedFrameRate: encodedFrameRate::<Identity, Impl, OFFSET>,
            framesSkipped: framesSkipped::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNetwork as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>() -> IWMPNodeRealEstate_Vtbl {
        unsafe extern "system" fn GetDesiredSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesiredSize(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn SetRects<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn GetRects<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn SetWindowless<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowless(::core::mem::transmute_copy(&fwindowless)).into()
        }
        unsafe extern "system" fn GetWindowless<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWindowless(::core::mem::transmute_copy(&pfwindowless)).into()
        }
        unsafe extern "system" fn SetFullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFullScreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn GetFullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFullScreen(::core::mem::transmute_copy(&pffullscreen)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDesiredSize: GetDesiredSize::<Identity, Impl, OFFSET>,
            SetRects: SetRects::<Identity, Impl, OFFSET>,
            GetRects: GetRects::<Identity, Impl, OFFSET>,
            SetWindowless: SetWindowless::<Identity, Impl, OFFSET>,
            GetWindowless: GetWindowless::<Identity, Impl, OFFSET>,
            SetFullScreen: SetFullScreen::<Identity, Impl, OFFSET>,
            GetFullScreen: GetFullScreen::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>() -> IWMPNodeRealEstateHost_Vtbl {
        unsafe extern "system" fn OnDesiredSizeChange<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDesiredSizeChange(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn OnFullScreenTransition<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFullScreenTransition(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnDesiredSizeChange: OnDesiredSizeChange::<Identity, Impl, OFFSET>,
            OnFullScreenTransition: OnFullScreenTransition::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>() -> IWMPNodeWindowed_Vtbl {
        unsafe extern "system" fn SetOwnerWindow<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerWindow(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetOwnerWindow<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOwnerWindow(::core::mem::transmute_copy(&phwnd)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetOwnerWindow: SetOwnerWindow::<Identity, Impl, OFFSET>,
            GetOwnerWindow: GetOwnerWindow::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: isize>() -> IWMPNodeWindowedHost_Vtbl {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWindowMessageFromRenderer(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnWindowMessageFromRenderer: OnWindowMessageFromRenderer::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowless_Impl, const OFFSET: isize>() -> IWMPNodeWindowless_Vtbl {
        unsafe extern "system" fn OnDraw<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDraw(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prcdraw)).into()
        }
        Self { base: IWMPWindowMessageSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnDraw: OnDraw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowless as ::windows::core::Interface>::IID || iid == &<IWMPWindowMessageSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessHost_Impl: Sized {
    fn InvalidateRect(&mut self, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowlessHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: isize>() -> IWMPNodeWindowlessHost_Vtbl {
        unsafe extern "system" fn InvalidateRect<Identity: ::windows::core::IUnknownImpl, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidateRect(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&ferase)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), InvalidateRect: InvalidateRect::<Identity, Impl, OFFSET> }
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
    fn SetuiMode(&mut self, bstrmode: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>() -> IWMPPlayer_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        Self {
            base: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(),
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID
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
    fn SetuiMode(&mut self, bstrmode: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>() -> IWMPPlayer2_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(),
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID
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
    fn SetuiMode(&mut self, bstrmode: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>() -> IWMPPlayer3_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IWMPCore2_Vtbl::new::<Identity, Impl, OFFSET>(),
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID || iid == &<IWMPCore2 as ::windows::core::Interface>::IID
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
    fn SetuiMode(&mut self, bstrmode: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&mut self, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn windowlessVideo(&mut self, pbenabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn isRemote(&mut self, pvarfisremote: *mut i16) -> ::windows::core::Result<()>;
    fn playerApplication(&mut self) -> ::windows::core::Result<IWMPPlayerApplication>;
    fn openPlayer(&mut self, bstrurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>() -> IWMPPlayer4_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetuiMode(::core::mem::transmute_copy(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn isRemote<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isRemote(::core::mem::transmute_copy(&pvarfisremote)).into()
        }
        unsafe extern "system" fn playerApplication<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).playerApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpplayerapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openPlayer<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).openPlayer(::core::mem::transmute_copy(&bstrurl)).into()
        }
        Self {
            base: IWMPCore3_Vtbl::new::<Identity, Impl, OFFSET>(),
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
            isRemote: isRemote::<Identity, Impl, OFFSET>,
            playerApplication: playerApplication::<Identity, Impl, OFFSET>,
            openPlayer: openPlayer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayer4 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPCore as ::windows::core::Interface>::IID || iid == &<IWMPCore2 as ::windows::core::Interface>::IID || iid == &<IWMPCore3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>() -> IWMPPlayerApplication_Vtbl {
        unsafe extern "system" fn switchToPlayerApplication<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).switchToPlayerApplication().into()
        }
        unsafe extern "system" fn switchToControl<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).switchToControl().into()
        }
        unsafe extern "system" fn playerDocked<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).playerDocked(::core::mem::transmute_copy(&pbplayerdocked)).into()
        }
        unsafe extern "system" fn hasDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).hasDisplay(::core::mem::transmute_copy(&pbhasdisplay)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            switchToPlayerApplication: switchToPlayerApplication::<Identity, Impl, OFFSET>,
            switchToControl: switchToControl::<Identity, Impl, OFFSET>,
            playerDocked: playerDocked::<Identity, Impl, OFFSET>,
            hasDisplay: hasDisplay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPlayerServices_Impl: Sized {
    fn activateUIPlugin(&mut self, bstrplugin: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPane(&mut self, bstrtaskpane: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPaneURL(&mut self, bstrtaskpane: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, bstrfriendlyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>() -> IWMPPlayerServices_Vtbl {
        unsafe extern "system" fn activateUIPlugin<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).activateUIPlugin(::core::mem::transmute_copy(&bstrplugin)).into()
        }
        unsafe extern "system" fn setTaskPane<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setTaskPane(::core::mem::transmute_copy(&bstrtaskpane)).into()
        }
        unsafe extern "system" fn setTaskPaneURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setTaskPaneURL(::core::mem::transmute_copy(&bstrtaskpane), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&bstrfriendlyname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            activateUIPlugin: activateUIPlugin::<Identity, Impl, OFFSET>,
            setTaskPane: setTaskPane::<Identity, Impl, OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPlayerServices2_Impl: Sized + IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(&mut self, bstrpriority: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPlayerServices2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices2_Impl, const OFFSET: isize>() -> IWMPPlayerServices2_Vtbl {
        unsafe extern "system" fn setBackgroundProcessingPriority<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlayerServices2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpriority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setBackgroundProcessingPriority(::core::mem::transmute_copy(&bstrpriority)).into()
        }
        Self {
            base: IWMPPlayerServices_Vtbl::new::<Identity, Impl, OFFSET>(),
            setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as ::windows::core::Interface>::IID || iid == &<IWMPPlayerServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylist_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn name(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn attributeName(&mut self, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32) -> ::windows::core::Result<IWMPMedia>;
    fn getItemInfo(&mut self, bstrname: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&mut self, bstrname: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn isIdentical(&mut self, piwmpplaylist: &::core::option::Option<IWMPPlaylist>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn clear(&mut self) -> ::windows::core::Result<()>;
    fn insertItem(&mut self, lindex: i32, piwmpmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn appendItem(&mut self, piwmpmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn removeItem(&mut self, piwmpmedia: &::core::option::Option<IWMPMedia>) -> ::windows::core::Result<()>;
    fn moveItem(&mut self, lindexold: i32, lindexnew: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>() -> IWMPPlaylist_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setname(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn attributeName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).attributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrattributename)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiwmpmedia = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpplaylist: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpplaylist), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn clear<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).clear().into()
        }
        unsafe extern "system" fn insertItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).insertItem(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn appendItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).appendItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn removeItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeItem(::core::mem::transmute(&piwmpmedia)).into()
        }
        unsafe extern "system" fn moveItem<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).moveItem(::core::mem::transmute_copy(&lindexold), ::core::mem::transmute_copy(&lindexnew)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Setname: Setname::<Identity, Impl, OFFSET>,
            attributeCount: attributeCount::<Identity, Impl, OFFSET>,
            attributeName: attributeName::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            setItemInfo: setItemInfo::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            insertItem: insertItem::<Identity, Impl, OFFSET>,
            appendItem: appendItem::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            moveItem: moveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylist as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistArray_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&mut self, lindex: i32) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>() -> IWMPPlaylistArray_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn newPlaylist(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getAll(&mut self) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn getByName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn remove(&mut self, pitem: &::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn setDeleted(&mut self, pitem: &::core::option::Option<IWMPPlaylist>, varfisdeleted: i16) -> ::windows::core::Result<()>;
    fn isDeleted(&mut self, pitem: &::core::option::Option<IWMPPlaylist>, pvarfisdeleted: *mut i16) -> ::windows::core::Result<()>;
    fn importPlaylist(&mut self, pitem: &::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>() -> IWMPPlaylistCollection_Vtbl {
        unsafe extern "system" fn newPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).newPlaylist(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppplaylistarray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppplaylistarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).remove(::core::mem::transmute(&pitem)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, varfisdeleted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, pvarfisdeleted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isDeleted(::core::mem::transmute(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        unsafe extern "system" fn importPlaylist<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr, ppimporteditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).importPlaylist(::core::mem::transmute(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimporteditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            newPlaylist: newPlaylist::<Identity, Impl, OFFSET>,
            getAll: getAll::<Identity, Impl, OFFSET>,
            getByName: getByName::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            setDeleted: setDeleted::<Identity, Impl, OFFSET>,
            isDeleted: isDeleted::<Identity, Impl, OFFSET>,
            importPlaylist: importPlaylist::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWMPPlugin_Impl: Sized {
    fn Init(&mut self, dwplaybackcontext: usize) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn GetID(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn AdviseWMPServices(&mut self, pwmpservices: &::core::option::Option<IWMPServices>) -> ::windows::core::Result<()>;
    fn UnAdviseWMPServices(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>() -> IWMPPlugin_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&dwplaybackcontext)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaps(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn AdviseWMPServices<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpservices: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdviseWMPServices(::core::mem::transmute(&pwmpservices)).into()
        }
        unsafe extern "system" fn UnAdviseWMPServices<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnAdviseWMPServices().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            AdviseWMPServices: AdviseWMPServices::<Identity, Impl, OFFSET>,
            UnAdviseWMPServices: UnAdviseWMPServices::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>() -> IWMPPluginEnable_Vtbl {
        unsafe extern "system" fn SetEnable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetEnable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEnable(::core::mem::transmute_copy(&pfenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetEnable: SetEnable::<Identity, Impl, OFFSET>,
            GetEnable: GetEnable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPluginEnable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWMPPluginUI_Impl: Sized {
    fn SetCore(&mut self, pcore: &::core::option::Option<IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn DisplayPropertyPage(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&mut self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWMPPluginUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>() -> IWMPPluginUI_Vtbl {
        unsafe extern "system" fn SetCore<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCore(::core::mem::transmute(&pcore)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&phwndwindow)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayPropertyPage(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetCore: SetCore::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPluginUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPQuery_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn addCondition(&mut self, bstrattribute: &super::super::Foundation::BSTR, bstroperator: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn beginNextGroup(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQuery_Impl, const OFFSET: isize>() -> IWMPQuery_Vtbl {
        unsafe extern "system" fn addCondition<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstroperator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addCondition(::core::mem::transmute_copy(&bstrattribute), ::core::mem::transmute_copy(&bstroperator), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn beginNextGroup<Identity: ::windows::core::IUnknownImpl, Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).beginNextGroup().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            addCondition: addCondition::<Identity, Impl, OFFSET>,
            beginNextGroup: beginNextGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPQuery as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>() -> IWMPRemoteMediaServices_Vtbl {
        unsafe extern "system" fn GetServiceType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetServiceType(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetApplicationName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetScriptableObject<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR, ppdispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScriptableObject(::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&ppdispatch)).into()
        }
        unsafe extern "system" fn GetCustomUIMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCustomUIMode(::core::mem::transmute_copy(&pbstrfile)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetServiceType: GetServiceType::<Identity, Impl, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, Impl, OFFSET>,
            GetScriptableObject: GetScriptableObject::<Identity, Impl, OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>() -> IWMPRenderConfig_Vtbl {
        unsafe extern "system" fn SetinProcOnly<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetinProcOnly(::core::mem::transmute_copy(&finproc)).into()
        }
        unsafe extern "system" fn inProcOnly<Identity: ::windows::core::IUnknownImpl, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).inProcOnly(::core::mem::transmute_copy(&pfinproc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetinProcOnly: SetinProcOnly::<Identity, Impl, OFFSET>,
            inProcOnly: inProcOnly::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServices_Impl, const OFFSET: isize>() -> IWMPServices_Vtbl {
        unsafe extern "system" fn GetStreamTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamTime(::core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetStreamState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamState(::core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStreamTime: GetStreamTime::<Identity, Impl, OFFSET>,
            GetStreamState: GetStreamState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn isAvailable(&mut self, bstritem: &super::super::Foundation::BSTR, pisavailable: *mut i16) -> ::windows::core::Result<()>;
    fn autoStart(&mut self, pfautostart: *mut i16) -> ::windows::core::Result<()>;
    fn SetautoStart(&mut self, fautostart: i16) -> ::windows::core::Result<()>;
    fn baseURL(&mut self, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetbaseURL(&mut self, bstrbaseurl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn defaultFrame(&mut self, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetdefaultFrame(&mut self, bstrdefaultframe: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn getMode(&mut self, bstrmode: &super::super::Foundation::BSTR, pvarfmode: *mut i16) -> ::windows::core::Result<()>;
    fn setMode(&mut self, bstrmode: &super::super::Foundation::BSTR, varfmode: i16) -> ::windows::core::Result<()>;
    fn enableErrorDialogs(&mut self, pfenableerrordialogs: *mut i16) -> ::windows::core::Result<()>;
    fn SetenableErrorDialogs(&mut self, fenableerrordialogs: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>() -> IWMPSettings_Vtbl {
        unsafe extern "system" fn isAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pisavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isAvailable(::core::mem::transmute_copy(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn autoStart<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautostart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).autoStart(::core::mem::transmute_copy(&pfautostart)).into()
        }
        unsafe extern "system" fn SetautoStart<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fautostart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetautoStart(::core::mem::transmute_copy(&fautostart)).into()
        }
        unsafe extern "system" fn baseURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).baseURL(::core::mem::transmute_copy(&pbstrbaseurl)).into()
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetbaseURL(::core::mem::transmute_copy(&bstrbaseurl)).into()
        }
        unsafe extern "system" fn defaultFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).defaultFrame(::core::mem::transmute_copy(&pbstrdefaultframe)).into()
        }
        unsafe extern "system" fn SetdefaultFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetdefaultFrame(::core::mem::transmute_copy(&bstrdefaultframe)).into()
        }
        unsafe extern "system" fn invokeURLs<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).invokeURLs(::core::mem::transmute_copy(&pfinvokeurls)).into()
        }
        unsafe extern "system" fn SetinvokeURLs<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finvokeurls: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetinvokeURLs(::core::mem::transmute_copy(&finvokeurls)).into()
        }
        unsafe extern "system" fn mute<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).mute(::core::mem::transmute_copy(&pfmute)).into()
        }
        unsafe extern "system" fn Setmute<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setmute(::core::mem::transmute_copy(&fmute)).into()
        }
        unsafe extern "system" fn playCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).playCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn SetplayCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetplayCount(::core::mem::transmute_copy(&lcount)).into()
        }
        unsafe extern "system" fn rate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).rate(::core::mem::transmute_copy(&pdrate)).into()
        }
        unsafe extern "system" fn Setrate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setrate(::core::mem::transmute_copy(&drate)).into()
        }
        unsafe extern "system" fn balance<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).balance(::core::mem::transmute_copy(&plbalance)).into()
        }
        unsafe extern "system" fn Setbalance<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setbalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn volume<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).volume(::core::mem::transmute_copy(&plvolume)).into()
        }
        unsafe extern "system" fn Setvolume<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setvolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn getMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarfmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getMode(::core::mem::transmute_copy(&bstrmode), ::core::mem::transmute_copy(&pvarfmode)).into()
        }
        unsafe extern "system" fn setMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varfmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setMode(::core::mem::transmute_copy(&bstrmode), ::core::mem::transmute_copy(&varfmode)).into()
        }
        unsafe extern "system" fn enableErrorDialogs<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).enableErrorDialogs(::core::mem::transmute_copy(&pfenableerrordialogs)).into()
        }
        unsafe extern "system" fn SetenableErrorDialogs<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetenableErrorDialogs(::core::mem::transmute_copy(&fenableerrordialogs)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAvailable: isAvailable::<Identity, Impl, OFFSET>,
            autoStart: autoStart::<Identity, Impl, OFFSET>,
            SetautoStart: SetautoStart::<Identity, Impl, OFFSET>,
            baseURL: baseURL::<Identity, Impl, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, Impl, OFFSET>,
            defaultFrame: defaultFrame::<Identity, Impl, OFFSET>,
            SetdefaultFrame: SetdefaultFrame::<Identity, Impl, OFFSET>,
            invokeURLs: invokeURLs::<Identity, Impl, OFFSET>,
            SetinvokeURLs: SetinvokeURLs::<Identity, Impl, OFFSET>,
            mute: mute::<Identity, Impl, OFFSET>,
            Setmute: Setmute::<Identity, Impl, OFFSET>,
            playCount: playCount::<Identity, Impl, OFFSET>,
            SetplayCount: SetplayCount::<Identity, Impl, OFFSET>,
            rate: rate::<Identity, Impl, OFFSET>,
            Setrate: Setrate::<Identity, Impl, OFFSET>,
            balance: balance::<Identity, Impl, OFFSET>,
            Setbalance: Setbalance::<Identity, Impl, OFFSET>,
            volume: volume::<Identity, Impl, OFFSET>,
            Setvolume: Setvolume::<Identity, Impl, OFFSET>,
            getMode: getMode::<Identity, Impl, OFFSET>,
            setMode: setMode::<Identity, Impl, OFFSET>,
            enableErrorDialogs: enableErrorDialogs::<Identity, Impl, OFFSET>,
            SetenableErrorDialogs: SetenableErrorDialogs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSettings as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettings2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPSettings_Impl {
    fn defaultAudioLanguage(&mut self, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn mediaAccessRights(&mut self, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn requestMediaAccessRights(&mut self, bstrdesiredaccess: &super::super::Foundation::BSTR, pvbaccepted: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2_Impl, const OFFSET: isize>() -> IWMPSettings2_Vtbl {
        unsafe extern "system" fn defaultAudioLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).defaultAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn mediaAccessRights<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).mediaAccessRights(::core::mem::transmute_copy(&pbstrrights)).into()
        }
        unsafe extern "system" fn requestMediaAccessRights<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvbaccepted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).requestMediaAccessRights(::core::mem::transmute_copy(&bstrdesiredaccess), ::core::mem::transmute_copy(&pvbaccepted)).into()
        }
        Self {
            base: IWMPSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            defaultAudioLanguage: defaultAudioLanguage::<Identity, Impl, OFFSET>,
            mediaAccessRights: mediaAccessRights::<Identity, Impl, OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSettings2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSkinManager_Impl: Sized {
    fn SetVisualStyle(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSkinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSkinManager_Impl, const OFFSET: isize>() -> IWMPSkinManager_Vtbl {
        unsafe extern "system" fn SetVisualStyle<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSkinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisualStyle(::core::mem::transmute_copy(&bstrpath)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetVisualStyle: SetVisualStyle::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection_Impl, const OFFSET: isize>() -> IWMPStringCollection_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWMPStringCollection_Impl {
    fn isIdentical(&mut self, piwmpstringcollection2: &::core::option::Option<IWMPStringCollection2>, pvbool: *mut i16) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, lcollectionindex: i32, bstritemname: &super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeCountByType(&mut self, lcollectionindex: i32, bstrtype: &super::super::Foundation::BSTR, bstrlanguage: &super::super::Foundation::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&mut self, lcollectionindex: i32, bstrtype: &super::super::Foundation::BSTR, bstrlanguage: &super::super::Foundation::BSTR, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>() -> IWMPStringCollection2_Vtbl {
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&piwmpstringcollection2), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAttributeCountByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows::core::IUnknownImpl, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfoByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute_copy(&bstrtype), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&lattributeindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base: IWMPStringCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWMPStringCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService_Impl: Sized {
    fn allowPlay(&mut self, hwnd: super::super::Foundation::HWND, pmedia: &::core::option::Option<IWMPMedia>, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowCDBurn(&mut self, hwnd: super::super::Foundation::HWND, pplaylist: &::core::option::Option<IWMPPlaylist>, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowPDATransfer(&mut self, hwnd: super::super::Foundation::HWND, pplaylist: &::core::option::Option<IWMPPlaylist>, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn startBackgroundProcessing(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>() -> IWMPSubscriptionService_Vtbl {
        unsafe extern "system" fn allowPlay<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: ::windows::core::RawPtr, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).allowPlay(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pmedia), ::core::mem::transmute_copy(&pfallowplay)).into()
        }
        unsafe extern "system" fn allowCDBurn<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).allowCDBurn(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pfallowburn)).into()
        }
        unsafe extern "system" fn allowPDATransfer<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: ::windows::core::RawPtr, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).allowPDATransfer(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pplaylist), ::core::mem::transmute_copy(&pfallowtransfer)).into()
        }
        unsafe extern "system" fn startBackgroundProcessing<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startBackgroundProcessing(::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            allowPlay: allowPlay::<Identity, Impl, OFFSET>,
            allowCDBurn: allowCDBurn::<Identity, Impl, OFFSET>,
            allowPDATransfer: allowPDATransfer::<Identity, Impl, OFFSET>,
            startBackgroundProcessing: startBackgroundProcessing::<Identity, Impl, OFFSET>,
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
    fn deviceAvailable(&mut self, bstrdevicename: &super::super::Foundation::BSTR, pcb: &::core::option::Option<IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
    fn prepareForSync(&mut self, bstrfilename: &super::super::Foundation::BSTR, bstrdevicename: &super::super::Foundation::BSTR, pcb: &::core::option::Option<IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>() -> IWMPSubscriptionService2_Vtbl {
        unsafe extern "system" fn stopBackgroundProcessing<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stopBackgroundProcessing().into()
        }
        unsafe extern "system" fn serviceEvent<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).serviceEvent(::core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn deviceAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deviceAvailable(::core::mem::transmute_copy(&bstrdevicename), ::core::mem::transmute(&pcb)).into()
        }
        unsafe extern "system" fn prepareForSync<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).prepareForSync(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&bstrdevicename), ::core::mem::transmute(&pcb)).into()
        }
        Self {
            base: IWMPSubscriptionService_Vtbl::new::<Identity, Impl, OFFSET>(),
            stopBackgroundProcessing: stopBackgroundProcessing::<Identity, Impl, OFFSET>,
            serviceEvent: serviceEvent::<Identity, Impl, OFFSET>,
            deviceAvailable: deviceAvailable::<Identity, Impl, OFFSET>,
            prepareForSync: prepareForSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionService2 as ::windows::core::Interface>::IID || iid == &<IWMPSubscriptionService as ::windows::core::Interface>::IID
    }
}
pub trait IWMPSubscriptionServiceCallback_Impl: Sized {
    fn onComplete(&mut self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWMPSubscriptionServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: isize>() -> IWMPSubscriptionServiceCallback_Vtbl {
        unsafe extern "system" fn onComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).onComplete(::core::mem::transmute_copy(&hrresult)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice_Impl: Sized {
    fn friendlyName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetfriendlyName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deviceName(&mut self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deviceId(&mut self, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn partnershipIndex(&mut self, plindex: *mut i32) -> ::windows::core::Result<()>;
    fn connected(&mut self, pvbconnected: *mut i16) -> ::windows::core::Result<()>;
    fn status(&mut self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()>;
    fn syncState(&mut self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()>;
    fn progress(&mut self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn createPartnership(&mut self, vbshowui: i16) -> ::windows::core::Result<()>;
    fn deletePartnership(&mut self) -> ::windows::core::Result<()>;
    fn start(&mut self) -> ::windows::core::Result<()>;
    fn stop(&mut self) -> ::windows::core::Result<()>;
    fn showSettings(&mut self) -> ::windows::core::Result<()>;
    fn isIdentical(&mut self, pdevice: &::core::option::Option<IWMPSyncDevice>, pvbool: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>() -> IWMPSyncDevice_Vtbl {
        unsafe extern "system" fn friendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).friendlyName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn SetfriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetfriendlyName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn deviceName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deviceName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn deviceId<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deviceId(::core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn partnershipIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).partnershipIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn connected<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).connected(::core::mem::transmute_copy(&pvbconnected)).into()
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).status(::core::mem::transmute_copy(&pwmpds)).into()
        }
        unsafe extern "system" fn syncState<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).syncState(::core::mem::transmute_copy(&pwmpss)).into()
        }
        unsafe extern "system" fn progress<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn createPartnership<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbshowui: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).createPartnership(::core::mem::transmute_copy(&vbshowui)).into()
        }
        unsafe extern "system" fn deletePartnership<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deletePartnership().into()
        }
        unsafe extern "system" fn start<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).start().into()
        }
        unsafe extern "system" fn stop<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).stop().into()
        }
        unsafe extern "system" fn showSettings<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).showSettings().into()
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pvbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).isIdentical(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            friendlyName: friendlyName::<Identity, Impl, OFFSET>,
            SetfriendlyName: SetfriendlyName::<Identity, Impl, OFFSET>,
            deviceName: deviceName::<Identity, Impl, OFFSET>,
            deviceId: deviceId::<Identity, Impl, OFFSET>,
            partnershipIndex: partnershipIndex::<Identity, Impl, OFFSET>,
            connected: connected::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            syncState: syncState::<Identity, Impl, OFFSET>,
            progress: progress::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            createPartnership: createPartnership::<Identity, Impl, OFFSET>,
            deletePartnership: deletePartnership::<Identity, Impl, OFFSET>,
            start: start::<Identity, Impl, OFFSET>,
            stop: stop::<Identity, Impl, OFFSET>,
            showSettings: showSettings::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice2_Impl: Sized + IWMPSyncDevice_Impl {
    fn setItemInfo(&mut self, bstritemname: &super::super::Foundation::BSTR, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice2_Impl, const OFFSET: isize>() -> IWMPSyncDevice2_Vtbl {
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setItemInfo(::core::mem::transmute_copy(&bstritemname), ::core::mem::transmute_copy(&bstrval)).into()
        }
        Self { base: IWMPSyncDevice_Vtbl::new::<Identity, Impl, OFFSET>(), setItemInfo: setItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as ::windows::core::Interface>::IID || iid == &<IWMPSyncDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSyncDevice3_Impl: Sized + IWMPSyncDevice_Impl + IWMPSyncDevice2_Impl {
    fn estimateSyncSize(&mut self, pnonruleplaylist: &::core::option::Option<IWMPPlaylist>, prulesplaylist: &::core::option::Option<IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cancelEstimation(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSyncDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>() -> IWMPSyncDevice3_Vtbl {
        unsafe extern "system" fn estimateSyncSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: ::windows::core::RawPtr, prulesplaylist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).estimateSyncSize(::core::mem::transmute(&pnonruleplaylist), ::core::mem::transmute(&prulesplaylist)).into()
        }
        unsafe extern "system" fn cancelEstimation<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).cancelEstimation().into()
        }
        Self {
            base: IWMPSyncDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Identity, Impl, OFFSET>,
            cancelEstimation: cancelEstimation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as ::windows::core::Interface>::IID || iid == &<IWMPSyncDevice as ::windows::core::Interface>::IID || iid == &<IWMPSyncDevice2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPSyncServices_Impl: Sized {
    fn deviceCount(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getDevice(&mut self, lindex: i32) -> ::windows::core::Result<IWMPSyncDevice>;
}
impl IWMPSyncServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServices_Impl, const OFFSET: isize>() -> IWMPSyncServices_Vtbl {
        unsafe extern "system" fn deviceCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deviceCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDevice(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            deviceCount: deviceCount::<Identity, Impl, OFFSET>,
            getDevice: getDevice::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPTranscodePolicy_Impl, const OFFSET: isize>() -> IWMPTranscodePolicy_Vtbl {
        unsafe extern "system" fn allowTranscode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPTranscodePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvballow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).allowTranscode(::core::mem::transmute_copy(&pvballow)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), allowTranscode: allowTranscode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPTranscodePolicy as ::windows::core::Interface>::IID
    }
}
pub trait IWMPUserEventSink_Impl: Sized {
    fn NotifyUserEvent(&mut self, eventcode: i32) -> ::windows::core::Result<()>;
}
impl IWMPUserEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPUserEventSink_Impl, const OFFSET: isize>() -> IWMPUserEventSink_Vtbl {
        unsafe extern "system" fn NotifyUserEvent<Identity: ::windows::core::IUnknownImpl, Impl: IWMPUserEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyUserEvent(::core::mem::transmute_copy(&eventcode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), NotifyUserEvent: NotifyUserEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPUserEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfig_Impl: Sized {
    fn SetpresenterActivate(&mut self, pactivate: &::core::option::Option<super::MediaFoundation::IMFActivate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IWMPVideoRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: isize>() -> IWMPVideoRenderConfig_Vtbl {
        unsafe extern "system" fn SetpresenterActivate<Identity: ::windows::core::IUnknownImpl, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetpresenterActivate(::core::mem::transmute(&pactivate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetpresenterActivate: SetpresenterActivate::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPWindowMessageSink_Impl, const OFFSET: isize>() -> IWMPWindowMessageSink_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows::core::IUnknownImpl, Impl: IWMPWindowMessageSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWindowMessage(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET> }
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
    fn Merge(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>() -> IXFeed_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&uiitemcount), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastwritetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *pfss = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyncSetting(::core::mem::transmute_copy(&fss)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *puiinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItem(::core::mem::transmute_copy(&uiid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MarkAllItemsRead().into()
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puimaxitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxItemCount(::core::mem::transmute_copy(&uimaxitemcount)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdownloadenclosuresautomatically = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&bdownloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *pfde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Merge(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&pszurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszhomepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszimageurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastbuilddate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstpubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    *puittl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlanguage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcopyright = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsList() {
                ::core::result::Result::Ok(ok__) => {
                    *pbislist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puiunreaditemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puiitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            SyncSetting: SyncSetting::<Identity, Impl, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, Impl, OFFSET>,
            Items: Items::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, Impl, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, Impl, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, Impl, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Ttl: Ttl::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Copyright: Copyright::<Identity, Impl, OFFSET>,
            IsList: IsList::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>() -> IXFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemByEffectiveId(::core::mem::transmute_copy(&uieffectiveid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastitemdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Username() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpassword = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearCredentials().into()
        }
        Self {
            base: IXFeed_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeed2 as ::windows::core::Interface>::IID || iid == &<IXFeed as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>() -> IXFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszmimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *puilength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    *pfde = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszmimetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdownloadurl: super::super::Foundation::PWSTR, pszdownloadfilepath: super::super::Foundation::PWSTR, pszdownloadmimetype: super::super::Foundation::PWSTR, pszenclosurefilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFile(::core::mem::transmute_copy(&pszdownloadurl), ::core::mem::transmute_copy(&pszdownloadfilepath), ::core::mem::transmute_copy(&pszdownloadmimetype), ::core::mem::transmute_copy(&pszenclosurefilename)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Url: Url::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            LocalPath: LocalPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, Impl, OFFSET>,
            RemoveFile: RemoveFile::<Identity, Impl, OFFSET>,
            SetFile: SetFile::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>() -> IXFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMoved(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMoved: FeedMoved::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>() -> IXFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfe = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExistsFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pbfeedexists)).into()
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExistsSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pbsubfolderexists)).into()
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeed(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubfolder(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisrootfeedfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puitotalunreaditemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *puitotalitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Feeds: Feeds::<Identity, Impl, OFFSET>,
            Subfolders: Subfolders::<Identity, Impl, OFFSET>,
            CreateFeed: CreateFeed::<Identity, Impl, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            IsRoot: IsRoot::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, Impl, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>() -> IXFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Error().into()
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderAdded(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderMovedFrom(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderMovedTo(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FolderItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedAdded(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDeleted(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedRenamed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedUrlChanged(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMovedFrom(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pszoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedMovedTo(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloading(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedDownloadCompleted(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FeedItemCountChanged(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, Impl, OFFSET>,
            FolderAdded: FolderAdded::<Identity, Impl, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, Impl, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, Impl, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, Impl, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, Impl, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, Impl, OFFSET>,
            FeedAdded: FeedAdded::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, Impl, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>() -> IXFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml(::core::mem::transmute_copy(&fxif)) {
                ::core::result::Result::Ok(ok__) => {
                    *pps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszguid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pstpubdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Comments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszauthor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enclosure(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRead<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsRead(::core::mem::transmute_copy(&bisread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *puiid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastdownloadtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *pstmodifiedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Comments: Comments::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Enclosure: Enclosure::<Identity, Impl, OFFSET>,
            IsRead: IsRead::<Identity, Impl, OFFSET>,
            SetIsRead: SetIsRead::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem2_Impl, const OFFSET: isize>() -> IXFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    *puieffectiveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IXFeedItem_Vtbl::new::<Identity, Impl, OFFSET>(), EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedItem2 as ::windows::core::Interface>::IID || iid == &<IXFeedItem as ::windows::core::Interface>::IID
    }
}
pub trait IXFeedsEnum_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IXFeedsEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnum_Impl, const OFFSET: isize>() -> IXFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *puicount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Count: Count::<Identity, Impl, OFFSET>, Item: Item::<Identity, Impl, OFFSET> }
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
    fn Normalize(&mut self, pstreamin: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn ItemCountLimit(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>() -> IXFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RootFolder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSubscribed(::core::mem::transmute_copy(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsubscribed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsFeed(::core::mem::transmute_copy(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfeedexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeed(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeedByUrl(::core::mem::transmute_copy(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExistsFolder(::core::mem::transmute_copy(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbfolderexists = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFolder(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFeed(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundSync(::core::mem::transmute_copy(&fbsa)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfbss = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *puiinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamin: ::windows::core::RawPtr, ppstreamout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Normalize(::core::mem::transmute(&pstreamin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstreamout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *puiitemcountlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RootFolder: RootFolder::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, Impl, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, Impl, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, Impl, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, Impl, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, Impl, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _WMPOCXEvents_Impl, const OFFSET: isize>() -> _WMPOCXEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_WMPOCXEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
