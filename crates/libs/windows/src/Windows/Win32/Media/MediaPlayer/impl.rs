#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Rename(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn LastWriteTime(&self) -> ::windows::core::Result<f64>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Download(&self) -> ::windows::core::Result<()>;
    fn AsyncDownload(&self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&self) -> ::windows::core::Result<()>;
    fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<i32>;
    fn SetInterval(&self, minutes: i32) -> ::windows::core::Result<()>;
    fn LastDownloadTime(&self) -> ::windows::core::Result<f64>;
    fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Items(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetItem(&self, itemid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Image(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn LastBuildDate(&self) -> ::windows::core::Result<f64>;
    fn PubDate(&self) -> ::windows::core::Result<f64>;
    fn Ttl(&self) -> ::windows::core::Result<i32>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Copyright(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MaxItemCount(&self) -> ::windows::core::Result<i32>;
    fn SetMaxItemCount(&self, count: i32) -> ::windows::core::Result<()>;
    fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDownloadEnclosuresAutomatically(&self, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&self, feedxml: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MarkAllItemsRead(&self) -> ::windows::core::Result<()>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn UnreadItemCount(&self) -> ::windows::core::Result<i32>;
    fn ItemCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeed {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>() -> IFeed_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Xml(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xml, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUrl(::core::mem::transmute(&feedurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwrite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(syncsetting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncSetting(::core::mem::transmute_copy(&syncsetting)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Interval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastdownload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Items() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItem(::core::mem::transmute_copy(&itemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homepage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(homepage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Image() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastbuilddate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastpopulatedate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ttl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyright: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copyright, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxItemCount(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(downloadenclosuresautomatically, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&downloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Merge(::core::mem::transmute(&feedxml), ::core::mem::transmute(&feedurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, islist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarkAllItemsRead().into()
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeed as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeed2_Impl: Sized + IFeed_Impl {
    fn GetItemByEffectiveId(&self, itemeffectiveid: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn LastItemDownloadTime(&self) -> ::windows::core::Result<f64>;
    fn Username(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCredentials(&self, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ClearCredentials(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeed2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeed2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>() -> IFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemByEffectiveId(::core::mem::transmute_copy(&itemeffectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastitemdownloadtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Username() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(username, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Password() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(password, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&username), ::core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearCredentials().into()
        }
        Self {
            base__: IFeed_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeed2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFeed as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedEnclosure_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn AsyncDownload(&self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&self) -> ::windows::core::Result<()>;
    fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn DownloadMimeType(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RemoveFile(&self) -> ::windows::core::Result<()>;
    fn SetFile(&self, downloadurl: &::windows::core::BSTR, downloadfilepath: &::windows::core::BSTR, downloadmimetype: &::windows::core::BSTR, enclosurefilename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedEnclosure {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEnclosure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>() -> IFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosureurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mimetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosureurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mimetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, downloadfilepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, downloadmimetype: ::std::mem::MaybeUninit<::windows::core::BSTR>, enclosurefilename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFile(::core::mem::transmute(&downloadurl), ::core::mem::transmute(&downloadfilepath), ::core::mem::transmute(&downloadmimetype), ::core::mem::transmute(&enclosurefilename)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedEnclosure as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&self) -> ::windows::core::Result<()>;
    fn FeedDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedMoved(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&self, path: &::windows::core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>() -> IFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDeleted(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedRenamed(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedUrlChanged(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMoved(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloading(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloadCompleted(::core::mem::transmute(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedItemCountChanged(::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedEvents as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Feeds(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Subfolders(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateFeed(&self, feedname: &::windows::core::BSTR, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn CreateSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFeed(&self, feedname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(&self, feedname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSubfolder(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Rename(&self, foldername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Move(&self, newparentpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsRoot(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TotalUnreadItemCount(&self) -> ::windows::core::Result<i32>;
    fn TotalItemCount(&self) -> ::windows::core::Result<i32>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedFolder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>() -> IFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows::core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFeed(::core::mem::transmute(&feedname), ::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSubfolder(::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows::core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsFeed(::core::mem::transmute(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeed(::core::mem::transmute(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows::core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsSubfolder(::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubfolder(::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foldername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute(&foldername)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(folderpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isroot: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isroot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedFolder as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedFolderEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&self) -> ::windows::core::Result<()>;
    fn FolderAdded(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FolderDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FolderRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedFrom(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FolderMovedTo(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FolderItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
    fn FeedAdded(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedDeleted(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedFrom(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedMovedTo(&self, path: &::windows::core::BSTR, oldpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&self, path: &::windows::core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&self, path: &::windows::core::BSTR, itemcounttype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedFolderEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedFolderEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>() -> IFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error().into()
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderAdded(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderDeleted(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderRenamed(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderMovedFrom(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderMovedTo(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderItemCountChanged(::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedAdded(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDeleted(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedRenamed(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedUrlChanged(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMovedFrom(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMovedTo(::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloading(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloadCompleted(::core::mem::transmute(&path), ::core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, itemcounttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedItemCountChanged(::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedFolderEvents as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Guid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn PubDate(&self) -> ::windows::core::Result<f64>;
    fn Comments(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Enclosure(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRead(&self, isread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn LocalId(&self) -> ::windows::core::Result<i32>;
    fn Parent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn LastDownloadTime(&self) -> ::windows::core::Result<f64>;
    fn Modified(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>() -> IFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Xml(::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xml, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linkurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linkurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Guid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pubdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comments: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Comments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Author() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(author, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enclosure() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isread, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsRead(::core::mem::transmute_copy(&isread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastdownload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedItem as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedItem2_Impl: Sized + IFeedItem_Impl {
    fn EffectiveId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem2_Impl, const OFFSET: isize>() -> IFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effectiveid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IFeedItem_Vtbl::new::<Identity, Impl, OFFSET>(), EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedItem2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFeedItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsEnum_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedsEnum {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: isize>() -> IFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvar, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedsEnum as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RootFolder(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn IsSubscribed(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExistsFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetFeedByUrl(&self, feedurl: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DeleteFeed(&self, feedpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DeleteFolder(&self, folderpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn BackgroundSync(&self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()>;
    fn BackgroundSyncStatus(&self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&self) -> ::windows::core::Result<i32>;
    fn SetDefaultInterval(&self, minutes: i32) -> ::windows::core::Result<()>;
    fn AsyncSyncAll(&self) -> ::windows::core::Result<()>;
    fn Normalize(&self, feedxmlin: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ItemCountLimit(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFeedsManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>() -> IFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootFolder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, subscribed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSubscribed(::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscribed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsFeed(::core::mem::transmute(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeed(::core::mem::transmute(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFeedByUrl(::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsFolder(::core::mem::transmute(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolder(::core::mem::transmute(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFeed(::core::mem::transmute(&feedpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFolder(::core::mem::transmute(&folderpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackgroundSync(::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedxmlin: ::std::mem::MaybeUninit<::windows::core::BSTR>, feedxmlout: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Normalize(::core::mem::transmute(&feedxmlin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedxmlout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemcountlimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IFeedsManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPAudioRenderConfig_Impl: Sized {
    fn audioOutputDevice(&self, pbstroutputdevice: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetaudioOutputDevice(&self, bstroutputdevice: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPAudioRenderConfig {}
impl IWMPAudioRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>() -> IWMPAudioRenderConfig_Vtbl {
        unsafe extern "system" fn audioOutputDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.audioOutputDevice(::core::mem::transmute_copy(&pbstroutputdevice)).into()
        }
        unsafe extern "system" fn SetaudioOutputDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetaudioOutputDevice(::core::mem::transmute(&bstroutputdevice)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            audioOutputDevice: audioOutputDevice::<Identity, Impl, OFFSET>,
            SetaudioOutputDevice: SetaudioOutputDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPAudioRenderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCdrom_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn driveSpecifier(&self, pbstrdrive: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn playlist(&self) -> ::windows::core::Result<IWMPPlaylist>;
    fn eject(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPCdrom {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdrom_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: isize>() -> IWMPCdrom_Vtbl {
        unsafe extern "system" fn driveSpecifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.driveSpecifier(::core::mem::transmute_copy(&pbstrdrive)).into()
        }
        unsafe extern "system" fn playlist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.playlist() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.eject().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            driveSpecifier: driveSpecifier::<Identity, Impl, OFFSET>,
            playlist: playlist::<Identity, Impl, OFFSET>,
            eject: eject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdrom as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPCdromBurn_Impl: Sized {
    fn isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getItemInfo(&self, bstritem: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn label(&self, pbstrlabel: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Setlabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn burnFormat(&self, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::Result<()>;
    fn SetburnFormat(&self, wmpbf: WMPBurnFormat) -> ::windows::core::Result<()>;
    fn burnPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetburnPlaylist(&self, pplaylist: ::core::option::Option<&IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn refreshStatus(&self) -> ::windows::core::Result<()>;
    fn burnState(&self, pwmpbs: *mut WMPBurnState) -> ::windows::core::Result<()>;
    fn burnProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startBurn(&self) -> ::windows::core::Result<()>;
    fn stopBurn(&self) -> ::windows::core::Result<()>;
    fn erase(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPCdromBurn {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPCdromBurn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>() -> IWMPCdromBurn_Vtbl {
        unsafe extern "system" fn isAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows::core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isAvailable(::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.label(::core::mem::transmute_copy(&pbstrlabel)).into()
        }
        unsafe extern "system" fn Setlabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setlabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn burnFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.burnFormat(::core::mem::transmute_copy(&pwmpbf)).into()
        }
        unsafe extern "system" fn SetburnFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetburnFormat(::core::mem::transmute_copy(&wmpbf)).into()
        }
        unsafe extern "system" fn burnPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.burnPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetburnPlaylist(::windows::core::from_raw_borrowed(&pplaylist)).into()
        }
        unsafe extern "system" fn refreshStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.refreshStatus().into()
        }
        unsafe extern "system" fn burnState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.burnState(::core::mem::transmute_copy(&pwmpbs)).into()
        }
        unsafe extern "system" fn burnProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.burnProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startBurn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startBurn().into()
        }
        unsafe extern "system" fn stopBurn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stopBurn().into()
        }
        unsafe extern "system" fn erase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.erase().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPCdromBurn as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCdromCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&self, lindex: i32) -> ::windows::core::Result<IWMPCdrom>;
    fn getByDriveSpecifier(&self, bstrdrivespecifier: &::windows::core::BSTR) -> ::windows::core::Result<IWMPCdrom>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPCdromCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCdromCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>() -> IWMPCdromCollection_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppcdrom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByDriveSpecifier(::core::mem::transmute(&bstrdrivespecifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcdrom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPCdromRip_Impl: Sized {
    fn ripState(&self, pwmprs: *mut WMPRipState) -> ::windows::core::Result<()>;
    fn ripProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startRip(&self) -> ::windows::core::Result<()>;
    fn stopRip(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPCdromRip {}
impl IWMPCdromRip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: isize>() -> IWMPCdromRip_Vtbl {
        unsafe extern "system" fn ripState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ripState(::core::mem::transmute_copy(&pwmprs)).into()
        }
        unsafe extern "system" fn ripProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ripProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startRip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startRip().into()
        }
        unsafe extern "system" fn stopRip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stopRip().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ripState: ripState::<Identity, Impl, OFFSET>,
            ripProgress: ripProgress::<Identity, Impl, OFFSET>,
            startRip: startRip::<Identity, Impl, OFFSET>,
            stopRip: stopRip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCdromRip as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPClosedCaption_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SAMIStyle(&self, pbstrsamistyle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIStyle(&self, bstrsamistyle: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SAMILang(&self, pbstrsamilang: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMILang(&self, bstrsamilang: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SAMIFileName(&self, pbstrsamifilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSAMIFileName(&self, bstrsamifilename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn captioningId(&self, pbstrcaptioningid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetcaptioningId(&self, bstrcaptioningid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPClosedCaption {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>() -> IWMPClosedCaption_Vtbl {
        unsafe extern "system" fn SAMIStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAMIStyle(::core::mem::transmute_copy(&pbstrsamistyle)).into()
        }
        unsafe extern "system" fn SetSAMIStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSAMIStyle(::core::mem::transmute(&bstrsamistyle)).into()
        }
        unsafe extern "system" fn SAMILang<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAMILang(::core::mem::transmute_copy(&pbstrsamilang)).into()
        }
        unsafe extern "system" fn SetSAMILang<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSAMILang(::core::mem::transmute(&bstrsamilang)).into()
        }
        unsafe extern "system" fn SAMIFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAMIFileName(::core::mem::transmute_copy(&pbstrsamifilename)).into()
        }
        unsafe extern "system" fn SetSAMIFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSAMIFileName(::core::mem::transmute(&bstrsamifilename)).into()
        }
        unsafe extern "system" fn captioningId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.captioningId(::core::mem::transmute_copy(&pbstrcaptioningid)).into()
        }
        unsafe extern "system" fn SetcaptioningId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcaptioningId(::core::mem::transmute(&bstrcaptioningid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPClosedCaption as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPClosedCaption2_Impl: Sized + IWMPClosedCaption_Impl {
    fn SAMILangCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getSAMILangName(&self, nindex: i32, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getSAMILangID(&self, nindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn SAMIStyleCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getSAMIStyleName(&self, nindex: i32, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPClosedCaption2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPClosedCaption2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>() -> IWMPClosedCaption2_Vtbl {
        unsafe extern "system" fn SAMILangCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAMILangCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMILangName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getSAMILangName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn getSAMILangID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getSAMILangID(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SAMIStyleCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAMIStyleCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMIStyleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getSAMIStyleName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        Self {
            base__: IWMPClosedCaption_Vtbl::new::<Identity, Impl, OFFSET>(),
            SAMILangCount: SAMILangCount::<Identity, Impl, OFFSET>,
            getSAMILangName: getSAMILangName::<Identity, Impl, OFFSET>,
            getSAMILangID: getSAMILangID::<Identity, Impl, OFFSET>,
            SAMIStyleCount: SAMIStyleCount::<Identity, Impl, OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPClosedCaption2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPClosedCaption as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPContentContainer_Impl: Sized {
    fn GetID(&self) -> ::windows::core::Result<u32>;
    fn GetPrice(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetType(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetContentCount(&self) -> ::windows::core::Result<u32>;
    fn GetContentPrice(&self, idxcontent: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetContentID(&self, idxcontent: u32) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IWMPContentContainer {}
impl IWMPContentContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>() -> IWMPContentContainer_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontentid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPrice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentPrice(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentID(::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontentid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetPrice: GetPrice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContentCount: GetContentCount::<Identity, Impl, OFFSET>,
            GetContentPrice: GetContentPrice::<Identity, Impl, OFFSET>,
            GetContentID: GetContentID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentContainer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPContentContainerList_Impl: Sized {
    fn GetTransactionType(&self) -> ::windows::core::Result<WMPTransactionType>;
    fn GetContainerCount(&self) -> ::windows::core::Result<u32>;
    fn GetContainer(&self, idxcontainer: u32) -> ::windows::core::Result<IWMPContentContainer>;
}
impl ::windows::core::RuntimeName for IWMPContentContainerList {}
impl IWMPContentContainerList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>() -> IWMPContentContainerList_Vtbl {
        unsafe extern "system" fn GetTransactionType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwmptt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccontainer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainer(::core::mem::transmute_copy(&idxcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransactionType: GetTransactionType::<Identity, Impl, OFFSET>,
            GetContainerCount: GetContainerCount::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPContentContainerList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPContentPartner_Impl: Sized {
    fn SetCallback(&self, pcallback: ::core::option::Option<&IWMPContentPartnerCallback>) -> ::windows::core::Result<()>;
    fn Notify(&self, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetItemInfo(&self, bstrinfoname: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetContentPartnerInfo(&self, bstrinfoname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetCommands(&self, location: &::windows::core::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &::windows::core::BSTR, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::Result<()>;
    fn InvokeCommand(&self, dwcommandid: u32, location: &::windows::core::BSTR, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: &::windows::core::BSTR, citemids: u32, rgitemids: *const u32) -> ::windows::core::Result<()>;
    fn CanBuySilent(&self, pinfo: ::core::option::Option<&IWMPContentContainerList>, pbstrtotalprice: *mut ::windows::core::BSTR, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Buy(&self, pinfo: ::core::option::Option<&IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn GetStreamingURL(&self, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Download(&self, pinfo: ::core::option::Option<&IWMPContentContainerList>, cookie: u32) -> ::windows::core::Result<()>;
    fn DownloadTrackComplete(&self, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn RefreshLicense(&self, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: &::windows::core::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: &::windows::core::BSTR, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCatalogURL(&self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::windows::core::BSTR, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetTemplate(&self, task: WMPTaskType, location: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, clicklocation: &::windows::core::BSTR, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: &::windows::core::BSTR, bstrviewparams: &::windows::core::BSTR, pbstrtemplateurl: *mut ::windows::core::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::Result<()>;
    fn UpdateDevice(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetListContents(&self, location: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: &::windows::core::BSTR, bstrparams: &::windows::core::BSTR, dwlistcookie: u32) -> ::windows::core::Result<()>;
    fn Login(&self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Authenticate(&self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB) -> ::windows::core::Result<()>;
    fn Logout(&self) -> ::windows::core::Result<()>;
    fn SendMessage(&self, bstrmsg: &::windows::core::BSTR, bstrparam: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StationEvent(&self, bstrstationeventtype: &::windows::core::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: &::windows::core::BSTR, dwsecondsplayed: u32) -> ::windows::core::Result<()>;
    fn CompareContainerListPrices(&self, plistbase: ::core::option::Option<&IWMPContentContainerList>, plistcompare: ::core::option::Option<&IWMPContentContainerList>) -> ::windows::core::Result<i32>;
    fn VerifyPermission(&self, bstrpermission: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPContentPartner {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>() -> IWMPContentPartner_Vtbl {
        unsafe extern "system" fn SetCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCallback(::windows::core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn GetItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontext: *const super::super::System::Com::VARIANT, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemInfo(::core::mem::transmute(&bstrinfoname), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPartnerInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentPartnerInfo(::core::mem::transmute(&bstrinfoname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows::core::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCommands(::core::mem::transmute(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&prgitemids), ::core::mem::transmute_copy(&pcitemids), ::core::mem::transmute_copy(&pprgitems)).into()
        }
        unsafe extern "system" fn InvokeCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::std::mem::MaybeUninit<::windows::core::BSTR>, plocationcontext: *const super::super::System::Com::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeCommand(::core::mem::transmute_copy(&dwcommandid), ::core::mem::transmute(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&rgitemids)).into()
        }
        unsafe extern "system" fn CanBuySilent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, pbstrtotalprice: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanBuySilent(::windows::core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&pbstrtotalprice), ::core::mem::transmute_copy(&psilentok)).into()
        }
        unsafe extern "system" fn Buy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Buy(::windows::core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetStreamingURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Com::VARIANT, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamingURL(::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&pstreamcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download(::windows::core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn DownloadTrackComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, contentid: u32, downloadtrackparam: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadTrackComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute(&downloadtrackparam)).into()
        }
        unsafe extern "system" fn RefreshLicense<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::std::mem::MaybeUninit<::windows::core::BSTR>, preasoncontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshLicense(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&flocal), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute(&bstrrefreshreason), ::core::mem::transmute_copy(&preasoncontext)).into()
        }
        unsafe extern "system" fn GetCatalogURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pexpirationdate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCatalogURL(::core::mem::transmute_copy(&dwcatalogversion), ::core::mem::transmute_copy(&dwcatalogschemaversion), ::core::mem::transmute_copy(&cataloglcid), ::core::mem::transmute_copy(&pdwnewcatalogversion), ::core::mem::transmute_copy(&pbstrcatalogurl), ::core::mem::transmute_copy(&pexpirationdate)).into()
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontext: *const super::super::System::Com::VARIANT, clicklocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, pclickcontext: *const super::super::System::Com::VARIANT, bstrfilter: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrviewparams: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrtemplateurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, ptemplatesize: *mut WMPTemplateSize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTemplate(::core::mem::transmute_copy(&task), ::core::mem::transmute(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&clicklocation), ::core::mem::transmute_copy(&pclickcontext), ::core::mem::transmute(&bstrfilter), ::core::mem::transmute(&bstrviewparams), ::core::mem::transmute_copy(&pbstrtemplateurl), ::core::mem::transmute_copy(&ptemplatesize)).into()
        }
        unsafe extern "system" fn UpdateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateDevice(::core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn GetListContents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontext: *const super::super::System::Com::VARIANT, bstrlisttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrparams: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwlistcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetListContents(::core::mem::transmute(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&bstrlisttype), ::core::mem::transmute(&bstrparams), ::core::mem::transmute_copy(&dwlistcookie)).into()
        }
        unsafe extern "system" fn Login<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Login(::core::mem::transmute(&userinfo), ::core::mem::transmute(&pwdinfo), ::core::mem::transmute_copy(&fusedcachedcreds), ::core::mem::transmute_copy(&foktocache)).into()
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Authenticate(::core::mem::transmute(&userinfo), ::core::mem::transmute(&pwdinfo)).into()
        }
        unsafe extern "system" fn Logout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Logout().into()
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessage(::core::mem::transmute(&bstrmsg), ::core::mem::transmute(&bstrparam)).into()
        }
        unsafe extern "system" fn StationEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsecondsplayed: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StationEvent(::core::mem::transmute(&bstrstationeventtype), ::core::mem::transmute_copy(&stationid), ::core::mem::transmute_copy(&playlistindex), ::core::mem::transmute_copy(&trackid), ::core::mem::transmute(&trackdata), ::core::mem::transmute_copy(&dwsecondsplayed)).into()
        }
        unsafe extern "system" fn CompareContainerListPrices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistbase: *mut ::core::ffi::c_void, plistcompare: *mut ::core::ffi::c_void, presult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareContainerListPrices(::windows::core::from_raw_borrowed(&plistbase), ::windows::core::from_raw_borrowed(&plistcompare)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermission<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VerifyPermission(::core::mem::transmute(&bstrpermission), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPContentPartner as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPContentPartnerCallback_Impl: Sized {
    fn Notify(&self, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BuyComplete(&self, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::Result<()>;
    fn DownloadTrack(&self, cookie: u32, bstrtrackurl: &::windows::core::BSTR, dwservicetrackid: u32, bstrdownloadparams: &::windows::core::BSTR, hrdownload: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetCatalogVersion(&self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn UpdateDeviceComplete(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ChangeView(&self, bstrtype: &::windows::core::BSTR, bstrid: &::windows::core::BSTR, bstrfilter: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddListContents(&self, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::Result<()>;
    fn ListContentsComplete(&self, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn SendMessageComplete(&self, bstrmsg: &::windows::core::BSTR, bstrparam: &::windows::core::BSTR, bstrresult: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetContentIDsInLibrary(&self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::Result<()>;
    fn RefreshLicenseComplete(&self, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ShowPopup(&self, lindex: i32, bstrparameters: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn VerifyPermissionComplete(&self, bstrpermission: &::windows::core::BSTR, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPContentPartnerCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPContentPartnerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>() -> IWMPContentPartnerCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn BuyComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwbuycookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BuyComplete(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwbuycookie)).into()
        }
        unsafe extern "system" fn DownloadTrack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::std::mem::MaybeUninit<::windows::core::BSTR>, hrdownload: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadTrack(::core::mem::transmute_copy(&cookie), ::core::mem::transmute(&bstrtrackurl), ::core::mem::transmute_copy(&dwservicetrackid), ::core::mem::transmute(&bstrdownloadparams), ::core::mem::transmute_copy(&hrdownload)).into()
        }
        unsafe extern "system" fn GetCatalogVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCatalogVersion(::core::mem::transmute_copy(&pdwversion), ::core::mem::transmute_copy(&pdwschemaversion), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn UpdateDeviceComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateDeviceComplete(::core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn ChangeView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrfilter: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeView(::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrid), ::core::mem::transmute(&bstrfilter)).into()
        }
        unsafe extern "system" fn AddListContents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddListContents(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&citems), ::core::mem::transmute_copy(&prgitems)).into()
        }
        unsafe extern "system" fn ListContentsComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ListContentsComplete(::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&hrsuccess)).into()
        }
        unsafe extern "system" fn SendMessageComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessageComplete(::core::mem::transmute(&bstrmsg), ::core::mem::transmute(&bstrparam), ::core::mem::transmute(&bstrresult)).into()
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContentIDsInLibrary(::core::mem::transmute_copy(&pccontentids), ::core::mem::transmute_copy(&pprgids)).into()
        }
        unsafe extern "system" fn RefreshLicenseComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshLicenseComplete(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&hrrefresh)).into()
        }
        unsafe extern "system" fn ShowPopup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowPopup(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrparameters)).into()
        }
        unsafe extern "system" fn VerifyPermissionComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcontext: *const super::super::System::Com::VARIANT, hrpermission: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VerifyPermissionComplete(::core::mem::transmute(&bstrpermission), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrpermission)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPContentPartnerCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn play(&self) -> ::windows::core::Result<()>;
    fn stop(&self) -> ::windows::core::Result<()>;
    fn pause(&self) -> ::windows::core::Result<()>;
    fn fastForward(&self) -> ::windows::core::Result<()>;
    fn fastReverse(&self) -> ::windows::core::Result<()>;
    fn currentPosition(&self, pdcurrentposition: *mut f64) -> ::windows::core::Result<()>;
    fn SetcurrentPosition(&self, dcurrentposition: f64) -> ::windows::core::Result<()>;
    fn currentPositionString(&self, pbstrcurrentposition: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn next(&self) -> ::windows::core::Result<()>;
    fn previous(&self) -> ::windows::core::Result<()>;
    fn currentItem(&self) -> ::windows::core::Result<IWMPMedia>;
    fn SetcurrentItem(&self, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn currentMarker(&self, plmarker: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentMarker(&self, lmarker: i32) -> ::windows::core::Result<()>;
    fn playItem(&self, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPControls {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>() -> IWMPControls_Vtbl {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows::core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_isAvailable(::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn play<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.play().into()
        }
        unsafe extern "system" fn stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stop().into()
        }
        unsafe extern "system" fn pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pause().into()
        }
        unsafe extern "system" fn fastForward<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fastForward().into()
        }
        unsafe extern "system" fn fastReverse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fastReverse().into()
        }
        unsafe extern "system" fn currentPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentPosition(::core::mem::transmute_copy(&pdcurrentposition)).into()
        }
        unsafe extern "system" fn SetcurrentPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentPosition(::core::mem::transmute_copy(&dcurrentposition)).into()
        }
        unsafe extern "system" fn currentPositionString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentPositionString(::core::mem::transmute_copy(&pbstrcurrentposition)).into()
        }
        unsafe extern "system" fn next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.next().into()
        }
        unsafe extern "system" fn previous<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.previous().into()
        }
        unsafe extern "system" fn currentItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.currentItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmedia, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentItem(::windows::core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn currentMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentMarker(::core::mem::transmute_copy(&plmarker)).into()
        }
        unsafe extern "system" fn SetcurrentMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentMarker(::core::mem::transmute_copy(&lmarker)).into()
        }
        unsafe extern "system" fn playItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.playItem(::windows::core::from_raw_borrowed(&piwmpmedia)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
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
        iid == &<IWMPControls as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls2_Impl: Sized + IWMPControls_Impl {
    fn step(&self, lstep: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPControls2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls2_Impl, const OFFSET: isize>() -> IWMPControls2_Vtbl {
        unsafe extern "system" fn step<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.step(::core::mem::transmute_copy(&lstep)).into()
        }
        Self { base__: IWMPControls_Vtbl::new::<Identity, Impl, OFFSET>(), step: step::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPControls2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPControls as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPControls3_Impl: Sized + IWMPControls2_Impl {
    fn audioLanguageCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getAudioLanguageID(&self, lindex: i32, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn getAudioLanguageDescription(&self, lindex: i32, pbstrlangdesc: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn currentAudioLanguage(&self, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentAudioLanguage(&self, llangid: i32) -> ::windows::core::Result<()>;
    fn currentAudioLanguageIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()>;
    fn SetcurrentAudioLanguageIndex(&self, lindex: i32) -> ::windows::core::Result<()>;
    fn getLanguageName(&self, llangid: i32, pbstrlangname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn currentPositionTimecode(&self, bstrtimecode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetcurrentPositionTimecode(&self, bstrtimecode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPControls3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPControls3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>() -> IWMPControls3_Vtbl {
        unsafe extern "system" fn audioLanguageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.audioLanguageCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAudioLanguageID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAudioLanguageID(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn getAudioLanguageDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAudioLanguageDescription(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrlangdesc)).into()
        }
        unsafe extern "system" fn currentAudioLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentAudioLanguage(::core::mem::transmute_copy(&llangid)).into()
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentAudioLanguageIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentAudioLanguageIndex(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn getLanguageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getLanguageName(::core::mem::transmute_copy(&llangid), ::core::mem::transmute_copy(&pbstrlangname)).into()
        }
        unsafe extern "system" fn currentPositionTimecode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentPositionTimecode(::core::mem::transmute_copy(&bstrtimecode)).into()
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentPositionTimecode(::core::mem::transmute(&bstrtimecode)).into()
        }
        Self {
            base__: IWMPControls2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPControls3 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPControls as ::windows::core::ComInterface>::IID || iid == &<IWMPControls2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPConvert_Impl: Sized {
    fn ConvertFile(&self, bstrinputfile: &::windows::core::BSTR, bstrdestinationfolder: &::windows::core::BSTR, pbstroutputfile: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetErrorURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPConvert {}
impl IWMPConvert_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: isize>() -> IWMPConvert_Vtbl {
        unsafe extern "system" fn ConvertFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdestinationfolder: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstroutputfile: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertFile(::core::mem::transmute(&bstrinputfile), ::core::mem::transmute(&bstrdestinationfolder), ::core::mem::transmute_copy(&pbstroutputfile)).into()
        }
        unsafe extern "system" fn GetErrorURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorURL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConvertFile: ConvertFile::<Identity, Impl, OFFSET>,
            GetErrorURL: GetErrorURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPConvert as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn close(&self) -> ::windows::core::Result<()>;
    fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn openState(&self, pwmpos: *mut WMPOpenState) -> ::windows::core::Result<()>;
    fn playState(&self, pwmpps: *mut WMPPlayState) -> ::windows::core::Result<()>;
    fn controls(&self) -> ::windows::core::Result<IWMPControls>;
    fn settings(&self) -> ::windows::core::Result<IWMPSettings>;
    fn currentMedia(&self) -> ::windows::core::Result<IWMPMedia>;
    fn SetcurrentMedia(&self, pmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection>;
    fn playlistCollection(&self) -> ::windows::core::Result<IWMPPlaylistCollection>;
    fn versionInfo(&self, pbstrversioninfo: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn launchURL(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn network(&self) -> ::windows::core::Result<IWMPNetwork>;
    fn currentPlaylist(&self) -> ::windows::core::Result<IWMPPlaylist>;
    fn SetcurrentPlaylist(&self, ppl: ::core::option::Option<&IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cdromCollection(&self) -> ::windows::core::Result<IWMPCdromCollection>;
    fn closedCaption(&self) -> ::windows::core::Result<IWMPClosedCaption>;
    fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn error(&self) -> ::windows::core::Result<IWMPError>;
    fn status(&self, pbstrstatus: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPCore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>() -> IWMPCore_Vtbl {
        unsafe extern "system" fn close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.close().into()
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn openState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.openState(::core::mem::transmute_copy(&pwmpos)).into()
        }
        unsafe extern "system" fn playState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.playState(::core::mem::transmute_copy(&pwmpps)).into()
        }
        unsafe extern "system" fn controls<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.controls() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontrol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn settings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.settings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.currentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmedia, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentMedia(::windows::core::from_raw_borrowed(&pmedia)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediacollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlistCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.playlistCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn versionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.versionInfo(::core::mem::transmute_copy(&pbstrversioninfo)).into()
        }
        unsafe extern "system" fn launchURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.launchURL(::core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn network<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqni: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.network() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqni, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.currentPlaylist() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcurrentPlaylist(::windows::core::from_raw_borrowed(&ppl)).into()
        }
        unsafe extern "system" fn cdromCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.cdromCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcdromcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn closedCaption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.closedCaption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclosedcaption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOnline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isOnline(::core::mem::transmute_copy(&pfonline)).into()
        }
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.status(::core::mem::transmute_copy(&pbstrstatus)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPCore as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore2_Impl: Sized + IWMPCore_Impl {
    fn dvd(&self) -> ::windows::core::Result<IWMPDVD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPCore2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore2_Impl, const OFFSET: isize>() -> IWMPCore2_Vtbl {
        unsafe extern "system" fn dvd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dvd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdvd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(), dvd: dvd::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPCore3_Impl: Sized + IWMPCore2_Impl {
    fn newPlaylist(&self, bstrname: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn newMedia(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPCore3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPCore3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: isize>() -> IWMPCore3_Vtbl {
        unsafe extern "system" fn newPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.newPlaylist(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn newMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.newMedia(::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmedia, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMPCore2_Vtbl::new::<Identity, Impl, OFFSET>(),
            newPlaylist: newPlaylist::<Identity, Impl, OFFSET>,
            newMedia: newMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPCore3 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID || iid == &<IWMPCore2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDVD_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn domain(&self, strdomain: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn topMenu(&self) -> ::windows::core::Result<()>;
    fn titleMenu(&self) -> ::windows::core::Result<()>;
    fn back(&self) -> ::windows::core::Result<()>;
    fn resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPDVD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDVD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>() -> IWMPDVD_Vtbl {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows::core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_isAvailable(::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn domain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.domain(::core::mem::transmute_copy(&strdomain)).into()
        }
        unsafe extern "system" fn topMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.topMenu().into()
        }
        unsafe extern "system" fn titleMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.titleMenu().into()
        }
        unsafe extern "system" fn back<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.back().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resume().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
            domain: domain::<Identity, Impl, OFFSET>,
            topMenu: topMenu::<Identity, Impl, OFFSET>,
            titleMenu: titleMenu::<Identity, Impl, OFFSET>,
            back: back::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDVD as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn id(&self, plid: *mut i32) -> ::windows::core::Result<()>;
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&self, litem: i32) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn startDownload(&self, bstrsourceurl: &::windows::core::BSTR, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPDownloadItem2>;
    fn removeItem(&self, litem: i32) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPDownloadCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>() -> IWMPDownloadCollection_Vtbl {
        unsafe extern "system" fn id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.id(::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.item(::core::mem::transmute_copy(&litem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdownload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.startDownload(::core::mem::transmute(&bstrsourceurl), ::core::mem::transmute(&bstrtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdownload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeItem(::core::mem::transmute_copy(&litem)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            id: id::<Identity, Impl, OFFSET>,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            startDownload: startDownload::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn sourceURL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn size(&self, plsize: *mut i32) -> ::windows::core::Result<()>;
    fn r#type(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::Result<()>;
    fn pause(&self) -> ::windows::core::Result<()>;
    fn resume(&self) -> ::windows::core::Result<()>;
    fn cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPDownloadItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>() -> IWMPDownloadItem_Vtbl {
        unsafe extern "system" fn sourceURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.sourceURL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.size(::core::mem::transmute_copy(&plsize)).into()
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.r#type(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn progress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn downloadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.downloadState(::core::mem::transmute_copy(&pwmpsdls)).into()
        }
        unsafe extern "system" fn pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pause().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resume().into()
        }
        unsafe extern "system" fn cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.cancel().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPDownloadItem as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadItem2_Impl: Sized + IWMPDownloadItem_Impl {
    fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPDownloadItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem2_Impl, const OFFSET: isize>() -> IWMPDownloadItem2_Vtbl {
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base__: IWMPDownloadItem_Vtbl::new::<Identity, Impl, OFFSET>(), getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadItem2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPDownloadItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPDownloadManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn getDownloadCollection(&self, lcollectionid: i32) -> ::windows::core::Result<IWMPDownloadCollection>;
    fn createDownloadCollection(&self) -> ::windows::core::Result<IWMPDownloadCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPDownloadManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPDownloadManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>() -> IWMPDownloadManager_Vtbl {
        unsafe extern "system" fn getDownloadCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDownloadCollection(::core::mem::transmute_copy(&lcollectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDownloadCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createDownloadCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getDownloadCollection: getDownloadCollection::<Identity, Impl, OFFSET>,
            createDownloadCollection: createDownloadCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPDownloadManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IWMPEffects_Impl: Sized {
    fn Render(&self, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn MediaInfo(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> ::windows::core::Result<()>;
    fn GetTitle(&self, bstrtitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetPresetCount(&self, pnpresetcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetCurrentPreset(&self, npreset: i32) -> ::windows::core::Result<()>;
    fn GetCurrentPreset(&self, pnpreset: *mut i32) -> ::windows::core::Result<()>;
    fn DisplayPropertyPage(&self, hwndowner: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GoFullscreen(&self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IWMPEffects {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IWMPEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>() -> IWMPEffects_Vtbl {
        unsafe extern "system" fn Render<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Render(::core::mem::transmute_copy(&plevels), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn MediaInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaInfo(::core::mem::transmute_copy(&lchannelcount), ::core::mem::transmute_copy(&lsamplerate), ::core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&pdwcapabilities)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetPresetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresetTitle(::core::mem::transmute_copy(&npreset), ::core::mem::transmute_copy(&bstrpresettitle)).into()
        }
        unsafe extern "system" fn GetPresetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresetCount(::core::mem::transmute_copy(&pnpresetcount)).into()
        }
        unsafe extern "system" fn SetCurrentPreset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentPreset(::core::mem::transmute_copy(&npreset)).into()
        }
        unsafe extern "system" fn GetCurrentPreset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPreset(::core::mem::transmute_copy(&pnpreset)).into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayPropertyPage(::core::mem::transmute_copy(&hwndowner)).into()
        }
        unsafe extern "system" fn GoFullscreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GoFullscreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn RenderFullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderFullScreen(::core::mem::transmute_copy(&plevels)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPEffects as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IWMPEffects2_Impl: Sized + IWMPEffects_Impl {
    fn SetCore(&self, pplayer: ::core::option::Option<&IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&self) -> ::windows::core::Result<()>;
    fn NotifyNewMedia(&self, pmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn OnWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn RenderWindowed(&self, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IWMPEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>() -> IWMPEffects2_Vtbl {
        unsafe extern "system" fn SetCore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCore(::windows::core::from_raw_borrowed(&pplayer)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn NotifyNewMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyNewMedia(::windows::core::from_raw_borrowed(&pmedia)).into()
        }
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresultparam)).into()
        }
        unsafe extern "system" fn RenderWindowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenderWindowed(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&frequiredrender)).into()
        }
        Self {
            base__: IWMPEffects_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetCore: SetCore::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            NotifyNewMedia: NotifyNewMedia::<Identity, Impl, OFFSET>,
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            RenderWindowed: RenderWindowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEffects2 as ::windows::core::ComInterface>::IID || iid == &<IWMPEffects as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPError_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn clearErrorQueue(&self) -> ::windows::core::Result<()>;
    fn errorCount(&self, plnumerrors: *mut i32) -> ::windows::core::Result<()>;
    fn get_item(&self, dwindex: i32) -> ::windows::core::Result<IWMPErrorItem>;
    fn webHelp(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPError {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: isize>() -> IWMPError_Vtbl {
        unsafe extern "system" fn clearErrorQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.clearErrorQueue().into()
        }
        unsafe extern "system" fn errorCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.errorCount(::core::mem::transmute_copy(&plnumerrors)).into()
        }
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperroritem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn webHelp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.webHelp().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            clearErrorQueue: clearErrorQueue::<Identity, Impl, OFFSET>,
            errorCount: errorCount::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            webHelp: webHelp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPError as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self, phr: *mut i32) -> ::windows::core::Result<()>;
    fn errorDescription(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn errorContext(&self, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn remedy(&self, plremedy: *mut i32) -> ::windows::core::Result<()>;
    fn customUrl(&self, pbstrcustomurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPErrorItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>() -> IWMPErrorItem_Vtbl {
        unsafe extern "system" fn errorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.errorCode(::core::mem::transmute_copy(&phr)).into()
        }
        unsafe extern "system" fn errorDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.errorDescription(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn errorContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.errorContext(::core::mem::transmute_copy(&pvarcontext)).into()
        }
        unsafe extern "system" fn remedy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remedy(::core::mem::transmute_copy(&plremedy)).into()
        }
        unsafe extern "system" fn customUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.customUrl(::core::mem::transmute_copy(&pbstrcustomurl)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            errorDescription: errorDescription::<Identity, Impl, OFFSET>,
            errorContext: errorContext::<Identity, Impl, OFFSET>,
            remedy: remedy::<Identity, Impl, OFFSET>,
            customUrl: customUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPErrorItem2_Impl: Sized + IWMPErrorItem_Impl {
    fn condition(&self, plcondition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPErrorItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPErrorItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem2_Impl, const OFFSET: isize>() -> IWMPErrorItem2_Vtbl {
        unsafe extern "system" fn condition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPErrorItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.condition(::core::mem::transmute_copy(&plcondition)).into()
        }
        Self { base__: IWMPErrorItem_Vtbl::new::<Identity, Impl, OFFSET>(), condition: condition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPErrorItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents_Impl: Sized {
    fn OpenStateChange(&self, newstate: i32);
    fn PlayStateChange(&self, newstate: i32);
    fn AudioLanguageChange(&self, langid: i32);
    fn StatusChange(&self);
    fn ScriptCommand(&self, sctype: &::windows::core::BSTR, param: &::windows::core::BSTR);
    fn NewStream(&self);
    fn Disconnect(&self, result: i32);
    fn Buffering(&self, start: super::super::Foundation::VARIANT_BOOL);
    fn Error(&self);
    fn Warning(&self, warningtype: i32, param: i32, description: &::windows::core::BSTR);
    fn EndOfStream(&self, result: i32);
    fn PositionChange(&self, oldposition: f64, newposition: f64);
    fn MarkerHit(&self, markernum: i32);
    fn DurationUnitChange(&self, newdurationunit: i32);
    fn CdromMediaChange(&self, cdromnum: i32);
    fn PlaylistChange(&self, playlist: ::core::option::Option<&super::super::System::Com::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(&self, bstritemname: &::windows::core::BSTR);
    fn MediaChange(&self, item: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CurrentMediaItemAvailable(&self, bstritemname: &::windows::core::BSTR);
    fn CurrentItemChange(&self, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionChange(&self);
    fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR);
    fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &::windows::core::BSTR, bstrattribval: &::windows::core::BSTR);
    fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &::windows::core::BSTR, bstroldattribval: &::windows::core::BSTR, bstrnewattribval: &::windows::core::BSTR);
    fn PlaylistCollectionChange(&self);
    fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &::windows::core::BSTR);
    fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &::windows::core::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &::windows::core::BSTR, varfisdeleted: super::super::Foundation::VARIANT_BOOL);
    fn ModeChange(&self, modename: &::windows::core::BSTR, newvalue: super::super::Foundation::VARIANT_BOOL);
    fn MediaError(&self, pmediaobject: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn OpenPlaylistSwitch(&self, pitem: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn DomainChange(&self, strdomain: &::windows::core::BSTR);
    fn SwitchedToPlayerApplication(&self);
    fn SwitchedToControl(&self);
    fn PlayerDockedStateChange(&self);
    fn PlayerReconnect(&self);
    fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn KeyDown(&self, nkeycode: i16, nshiftstate: i16);
    fn KeyPress(&self, nkeyascii: i16);
    fn KeyUp(&self, nkeycode: i16, nshiftstate: i16);
    fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>() -> IWMPEvents_Vtbl {
        unsafe extern "system" fn OpenStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn PlayStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlayStateChange(::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn AudioLanguageChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AudioLanguageChange(::core::mem::transmute_copy(&langid))
        }
        unsafe extern "system" fn StatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StatusChange()
        }
        unsafe extern "system" fn ScriptCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sctype: ::std::mem::MaybeUninit<::windows::core::BSTR>, param: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScriptCommand(::core::mem::transmute(&sctype), ::core::mem::transmute(&param))
        }
        unsafe extern "system" fn NewStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewStream()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn Buffering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: super::super::Foundation::VARIANT_BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Buffering(::core::mem::transmute_copy(&start))
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error()
        }
        unsafe extern "system" fn Warning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Warning(::core::mem::transmute_copy(&warningtype), ::core::mem::transmute_copy(&param), ::core::mem::transmute(&description))
        }
        unsafe extern "system" fn EndOfStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndOfStream(::core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn PositionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PositionChange(::core::mem::transmute_copy(&oldposition), ::core::mem::transmute_copy(&newposition))
        }
        unsafe extern "system" fn MarkerHit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarkerHit(::core::mem::transmute_copy(&markernum))
        }
        unsafe extern "system" fn DurationUnitChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DurationUnitChange(::core::mem::transmute_copy(&newdurationunit))
        }
        unsafe extern "system" fn CdromMediaChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromMediaChange(::core::mem::transmute_copy(&cdromnum))
        }
        unsafe extern "system" fn PlaylistChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlist: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlaylistChange(::windows::core::from_raw_borrowed(&playlist), ::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CurrentPlaylistChange(::core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CurrentPlaylistItemAvailable(::core::mem::transmute(&bstritemname))
        }
        unsafe extern "system" fn MediaChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaChange(::windows::core::from_raw_borrowed(&item))
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CurrentMediaItemAvailable(::core::mem::transmute(&bstritemname))
        }
        unsafe extern "system" fn CurrentItemChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CurrentItemChange(::windows::core::from_raw_borrowed(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionChange()
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionAttributeStringAdded(::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionAttributeStringRemoved(::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstroldattribval: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrnewattribval: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionAttributeStringChanged(::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstroldattribval), ::core::mem::transmute(&bstrnewattribval))
        }
        unsafe extern "system" fn PlaylistCollectionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlaylistCollectionChange()
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlaylistCollectionPlaylistAdded(::core::mem::transmute(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlaylistCollectionPlaylistRemoved(::core::mem::transmute(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlaylistCollectionPlaylistSetAsDeleted(::core::mem::transmute(&bstrplaylistname), ::core::mem::transmute_copy(&varfisdeleted))
        }
        unsafe extern "system" fn ModeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modename: ::std::mem::MaybeUninit<::windows::core::BSTR>, newvalue: super::super::Foundation::VARIANT_BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModeChange(::core::mem::transmute(&modename), ::core::mem::transmute_copy(&newvalue))
        }
        unsafe extern "system" fn MediaError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediaobject: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaError(::windows::core::from_raw_borrowed(&pmediaobject))
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenPlaylistSwitch(::windows::core::from_raw_borrowed(&pitem))
        }
        unsafe extern "system" fn DomainChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdomain: ::std::mem::MaybeUninit<::windows::core::BSTR>) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DomainChange(::core::mem::transmute(&strdomain))
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchedToPlayerApplication()
        }
        unsafe extern "system" fn SwitchedToControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchedToControl()
        }
        unsafe extern "system" fn PlayerDockedStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlayerDockedStateChange()
        }
        unsafe extern "system" fn PlayerReconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlayerReconnect()
        }
        unsafe extern "system" fn Click<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Click(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn DoubleClick<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoubleClick(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KeyDown(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn KeyPress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KeyPress(::core::mem::transmute_copy(&nkeyascii))
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KeyUp(::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn MouseDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MouseDown(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseMove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MouseMove(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MouseUp(::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents2_Impl: Sized + IWMPEvents_Impl {
    fn DeviceConnect(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>);
    fn DeviceDisconnect(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>);
    fn DeviceStatusChange(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CreatePartnershipComplete(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, hrresult: ::windows::core::HRESULT);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPEvents2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>() -> IWMPEvents2_Vtbl {
        unsafe extern "system" fn DeviceConnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceConnect(::windows::core::from_raw_borrowed(&pdevice))
        }
        unsafe extern "system" fn DeviceDisconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceDisconnect(::windows::core::from_raw_borrowed(&pdevice))
        }
        unsafe extern "system" fn DeviceStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstatus: WMPDeviceStatus) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceStatusChange(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&newstatus))
        }
        unsafe extern "system" fn DeviceSyncStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstate: WMPSyncState) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceSyncStateChange(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn DeviceSyncError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceSyncError(::windows::core::from_raw_borrowed(&pdevice), ::windows::core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CreatePartnershipComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePartnershipComplete(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hrresult))
        }
        Self {
            base__: IWMPEvents_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeviceConnect: DeviceConnect::<Identity, Impl, OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Identity, Impl, OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Identity, Impl, OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Identity, Impl, OFFSET>,
            DeviceSyncError: DeviceSyncError::<Identity, Impl, OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents2 as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents3_Impl: Sized + IWMPEvents2_Impl {
    fn CdromRipStateChange(&self, pcdromrip: ::core::option::Option<&IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(&self, pcdromrip: ::core::option::Option<&IWMPCdromRip>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnStateChange(&self, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(&self, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnError(&self, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, hrerror: ::windows::core::HRESULT);
    fn LibraryConnect(&self, plibrary: ::core::option::Option<&IWMPLibrary>);
    fn LibraryDisconnect(&self, plibrary: ::core::option::Option<&IWMPLibrary>);
    fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(&self, pdispstringcollection: ::core::option::Option<&super::super::System::Com::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(&self, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionMediaRemoved(&self, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPEvents3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>() -> IWMPEvents3_Vtbl {
        unsafe extern "system" fn CdromRipStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, wmprs: WMPRipState) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromRipStateChange(::windows::core::from_raw_borrowed(&pcdromrip), ::core::mem::transmute_copy(&wmprs))
        }
        unsafe extern "system" fn CdromRipMediaError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromRipMediaError(::windows::core::from_raw_borrowed(&pcdromrip), ::windows::core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CdromBurnStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, wmpbs: WMPBurnState) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromBurnStateChange(::windows::core::from_raw_borrowed(&pcdromburn), ::core::mem::transmute_copy(&wmpbs))
        }
        unsafe extern "system" fn CdromBurnMediaError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromBurnMediaError(::windows::core::from_raw_borrowed(&pcdromburn), ::windows::core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CdromBurnError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CdromBurnError(::windows::core::from_raw_borrowed(&pcdromburn), ::core::mem::transmute_copy(&hrerror))
        }
        unsafe extern "system" fn LibraryConnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LibraryConnect(::windows::core::from_raw_borrowed(&plibrary))
        }
        unsafe extern "system" fn LibraryDisconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LibraryDisconnect(::windows::core::from_raw_borrowed(&plibrary))
        }
        unsafe extern "system" fn FolderScanStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderScanStateChange(::core::mem::transmute_copy(&wmpfss))
        }
        unsafe extern "system" fn StringCollectionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispstringcollection: *mut ::core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StringCollectionChange(::windows::core::from_raw_borrowed(&pdispstringcollection), ::core::mem::transmute_copy(&change), ::core::mem::transmute_copy(&lcollectionindex))
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionMediaAdded(::windows::core::from_raw_borrowed(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MediaCollectionMediaRemoved(::windows::core::from_raw_borrowed(&pdispmedia))
        }
        Self {
            base__: IWMPEvents2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPEvents3 as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents4_Impl: Sized + IWMPEvents3_Impl {
    fn DeviceEstimation(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPEvents4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPEvents4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents4_Impl, const OFFSET: isize>() -> IWMPEvents4_Vtbl {
        unsafe extern "system" fn DeviceEstimation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPEvents4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceEstimation(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&qwestimatedusedspace), ::core::mem::transmute_copy(&qwestimatedspace))
        }
        Self { base__: IWMPEvents3_Vtbl::new::<Identity, Impl, OFFSET>(), DeviceEstimation: DeviceEstimation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPEvents4 as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents2 as ::windows::core::ComInterface>::IID || iid == &<IWMPEvents3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPFolderMonitorServices_Impl: Sized {
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&self, lindex: i32, pbstrfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn add(&self, bstrfolder: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn remove(&self, lindex: i32) -> ::windows::core::Result<()>;
    fn scanState(&self, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::Result<()>;
    fn currentFolder(&self, pbstrfolder: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn scannedFilesCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn addedFilesCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn updateProgress(&self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn startScan(&self) -> ::windows::core::Result<()>;
    fn stopScan(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPFolderMonitorServices {}
impl IWMPFolderMonitorServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>() -> IWMPFolderMonitorServices_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfolder: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.add(::core::mem::transmute(&bstrfolder)).into()
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn scanState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.scanState(::core::mem::transmute_copy(&pwmpfss)).into()
        }
        unsafe extern "system" fn currentFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.currentFolder(::core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn scannedFilesCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.scannedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn addedFilesCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addedFilesCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn updateProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.updateProgress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startScan<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startScan().into()
        }
        unsafe extern "system" fn stopScan<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stopScan().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPFolderMonitorServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPGraphCreation_Impl: Sized {
    fn GraphCreationPreRender(&self, pfiltergraph: ::core::option::Option<&::windows::core::IUnknown>, preserved: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GraphCreationPostRender(&self, pfiltergraph: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetGraphCreationFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPGraphCreation {}
impl IWMPGraphCreation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>() -> IWMPGraphCreation_Vtbl {
        unsafe extern "system" fn GraphCreationPreRender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GraphCreationPreRender(::windows::core::from_raw_borrowed(&pfiltergraph), ::windows::core::from_raw_borrowed(&preserved)).into()
        }
        unsafe extern "system" fn GraphCreationPostRender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GraphCreationPostRender(::windows::core::from_raw_borrowed(&pfiltergraph)).into()
        }
        unsafe extern "system" fn GetGraphCreationFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGraphCreationFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GraphCreationPreRender: GraphCreationPreRender::<Identity, Impl, OFFSET>,
            GraphCreationPostRender: GraphCreationPostRender::<Identity, Impl, OFFSET>,
            GetGraphCreationFlags: GetGraphCreationFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPGraphCreation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary_Impl: Sized {
    fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn r#type(&self, pwmplt: *mut WMPLibraryType) -> ::windows::core::Result<()>;
    fn mediaCollection(&self) -> ::windows::core::Result<IWMPMediaCollection>;
    fn isIdentical(&self, piwmplibrary: ::core::option::Option<&IWMPLibrary>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPLibrary {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: isize>() -> IWMPLibrary_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.r#type(::core::mem::transmute_copy(&pwmplt)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mediaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmediacollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmplibrary: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isIdentical(::windows::core::from_raw_borrowed(&piwmplibrary), ::core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            mediaCollection: mediaCollection::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary2_Impl: Sized + IWMPLibrary_Impl {
    fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPLibrary2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPLibrary2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary2_Impl, const OFFSET: isize>() -> IWMPLibrary2_Vtbl {
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrary2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base__: IWMPLibrary_Vtbl::new::<Identity, Impl, OFFSET>(), getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrary2 as ::windows::core::ComInterface>::IID || iid == &<IWMPLibrary as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPLibraryServices_Impl: Sized {
    fn getCountByType(&self, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> ::windows::core::Result<IWMPLibrary>;
}
impl ::windows::core::RuntimeName for IWMPLibraryServices {}
impl IWMPLibraryServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>() -> IWMPLibraryServices_Vtbl {
        unsafe extern "system" fn getCountByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getCountByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getLibraryByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLibraryByType(::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmplibrary, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getCountByType: getCountByType::<Identity, Impl, OFFSET>,
            getLibraryByType: getLibraryByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibraryServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPLibrarySharingServices_Impl: Sized {
    fn isLibraryShared(&self, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn isLibrarySharingEnabled(&self, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn showLibrarySharing(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPLibrarySharingServices {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPLibrarySharingServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>() -> IWMPLibrarySharingServices_Vtbl {
        unsafe extern "system" fn isLibraryShared<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isLibraryShared(::core::mem::transmute_copy(&pvbshared)).into()
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isLibrarySharingEnabled(::core::mem::transmute_copy(&pvbenabled)).into()
        }
        unsafe extern "system" fn showLibrarySharing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.showLibrarySharing().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            isLibraryShared: isLibraryShared::<Identity, Impl, OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Identity, Impl, OFFSET>,
            showLibrarySharing: showLibrarySharing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPLibrarySharingServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isIdentical(&self, piwmpmedia: ::core::option::Option<&IWMPMedia>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn sourceURL(&self, pbstrsourceurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn imageSourceWidth(&self, pwidth: *mut i32) -> ::windows::core::Result<()>;
    fn imageSourceHeight(&self, pheight: *mut i32) -> ::windows::core::Result<()>;
    fn markerCount(&self, pmarkercount: *mut i32) -> ::windows::core::Result<()>;
    fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::Result<()>;
    fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn duration(&self, pduration: *mut f64) -> ::windows::core::Result<()>;
    fn durationString(&self, pbstrduration: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getAttributeName(&self, lindex: i32, pbstritemname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn isMemberOf(&self, pplaylist: ::core::option::Option<&IWMPPlaylist>, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn isReadOnlyItem(&self, bstritemname: &::windows::core::BSTR, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMedia {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>() -> IWMPMedia_Vtbl {
        unsafe extern "system" fn get_isIdentical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_isIdentical(::windows::core::from_raw_borrowed(&piwmpmedia), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn sourceURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.sourceURL(::core::mem::transmute_copy(&pbstrsourceurl)).into()
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setname(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn imageSourceWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.imageSourceWidth(::core::mem::transmute_copy(&pwidth)).into()
        }
        unsafe extern "system" fn imageSourceHeight<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.imageSourceHeight(::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn markerCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.markerCount(::core::mem::transmute_copy(&pmarkercount)).into()
        }
        unsafe extern "system" fn getMarkerTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getMarkerTime(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pmarkertime)).into()
        }
        unsafe extern "system" fn getMarkerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getMarkerName(::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pbstrmarkername)).into()
        }
        unsafe extern "system" fn duration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn durationString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.durationString(::core::mem::transmute_copy(&pbstrduration)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAttributeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAttributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstritemname)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn getItemInfoByAtom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfoByAtom(::core::mem::transmute_copy(&latom), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn isMemberOf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isMemberOf(::windows::core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pvarfismemberof)).into()
        }
        unsafe extern "system" fn isReadOnlyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isReadOnlyItem(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pvarfisreadonly)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_isIdentical: get_isIdentical::<Identity, Impl, OFFSET>,
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
        iid == &<IWMPMedia as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia2_Impl: Sized + IWMPMedia_Impl {
    fn error(&self) -> ::windows::core::Result<IWMPErrorItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMedia2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia2_Impl, const OFFSET: isize>() -> IWMPMedia2_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmperroritem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWMPMedia_Vtbl::new::<Identity, Impl, OFFSET>(), error: error::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPMedia as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMedia3_Impl: Sized + IWMPMedia2_Impl {
    fn getAttributeCountByType(&self, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&self, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMedia3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMedia3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: isize>() -> IWMPMedia3_Vtbl {
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAttributeCountByType(::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfoByType(::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: IWMPMedia2_Vtbl::new::<Identity, Impl, OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMedia3 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPMedia as ::windows::core::ComInterface>::IID || iid == &<IWMPMedia2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn add(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<IWMPMedia>;
    fn getAll(&self) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByGenre(&self, bstrgenre: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAuthor(&self, bstrauthor: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAlbum(&self, bstralbum: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getByAttribute(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn remove(&self, pitem: ::core::option::Option<&IWMPMedia>, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getAttributeStringCollection(&self, bstrattribute: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPStringCollection>;
    fn getMediaAtom(&self, bstritemname: &::windows::core::BSTR, platom: *mut i32) -> ::windows::core::Result<()>;
    fn setDeleted(&self, pitem: ::core::option::Option<&IWMPMedia>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn isDeleted(&self, pitem: ::core::option::Option<&IWMPMedia>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMediaCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>() -> IWMPMediaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.add(::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAll() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByName(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByGenre<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgenre: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByGenre(::core::mem::transmute(&bstrgenre)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAuthor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthor: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByAuthor(::core::mem::transmute(&bstrauthor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAlbum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstralbum: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByAlbum(::core::mem::transmute(&bstralbum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByAttribute(::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remove(::windows::core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfdeletefile)).into()
        }
        unsafe extern "system" fn getAttributeStringCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttributeStringCollection(::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstringcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMediaAtom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, platom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getMediaAtom(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&platom)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setDeleted(::windows::core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isDeleted(::windows::core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPMediaCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMediaCollection2_Impl: Sized + IWMPMediaCollection_Impl {
    fn createQuery(&self) -> ::windows::core::Result<IWMPQuery>;
    fn getPlaylistByQuery(&self, pquery: ::core::option::Option<&IWMPQuery>, bstrmediatype: &::windows::core::BSTR, bstrsortattribute: &::windows::core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(&self, bstrattribute: &::windows::core::BSTR, pquery: ::core::option::Option<&IWMPQuery>, bstrmediatype: &::windows::core::BSTR, bstrsortattribute: &::windows::core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(&self, bstrattribute: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR, bstrmediatype: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMediaCollection2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMediaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>() -> IWMPMediaCollection2_Vtbl {
        unsafe extern "system" fn createQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getPlaylistByQuery(::windows::core::from_raw_borrowed(&pquery), ::core::mem::transmute(&bstrmediatype), ::core::mem::transmute(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getStringCollectionByQuery(::core::mem::transmute(&bstrattribute), ::windows::core::from_raw_borrowed(&pquery), ::core::mem::transmute(&bstrmediatype), ::core::mem::transmute(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstringcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByAttributeAndMediaType(::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrvalue), ::core::mem::transmute(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMPMediaCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            createQuery: createQuery::<Identity, Impl, OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Identity, Impl, OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Identity, Impl, OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaCollection2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPMediaCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPMediaPluginRegistrar_Impl: Sized {
    fn WMPRegisterPlayerPlugin(&self, pwszfriendlyname: &::windows::core::PCWSTR, pwszdescription: &::windows::core::PCWSTR, pwszuninstallstring: &::windows::core::PCWSTR, dwpriority: u32, guidplugintype: &::windows::core::GUID, clsid: &::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn WMPUnRegisterPlayerPlugin(&self, guidplugintype: &::windows::core::GUID, clsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPMediaPluginRegistrar {}
impl IWMPMediaPluginRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>() -> IWMPMediaPluginRegistrar_Vtbl {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR, pwszdescription: ::windows::core::PCWSTR, pwszuninstallstring: ::windows::core::PCWSTR, dwpriority: u32, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMPRegisterPlayerPlugin(::core::mem::transmute(&pwszfriendlyname), ::core::mem::transmute(&pwszdescription), ::core::mem::transmute(&pwszuninstallstring), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute(&guidplugintype), ::core::mem::transmute(&clsid), ::core::mem::transmute_copy(&cmediatypes), ::core::mem::transmute_copy(&pmediatypes)).into()
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMPUnRegisterPlayerPlugin(::core::mem::transmute(&guidplugintype), ::core::mem::transmute(&clsid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WMPRegisterPlayerPlugin: WMPRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
            WMPUnRegisterPlayerPlugin: WMPUnRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMediaPluginRegistrar as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMetadataPicture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn mimeType(&self, pbstrmimetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn pictureType(&self, pbstrpicturetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn description(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn URL(&self, pbstrurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMetadataPicture {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataPicture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>() -> IWMPMetadataPicture_Vtbl {
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.mimeType(::core::mem::transmute_copy(&pbstrmimetype)).into()
        }
        unsafe extern "system" fn pictureType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pictureType(::core::mem::transmute_copy(&pbstrpicturetype)).into()
        }
        unsafe extern "system" fn description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.URL(::core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            pictureType: pictureType::<Identity, Impl, OFFSET>,
            description: description::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataPicture as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPMetadataText_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn description(&self, pbstrdescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn text(&self, pbstrtext: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPMetadataText {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPMetadataText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: isize>() -> IWMPMetadataText_Vtbl {
        unsafe extern "system" fn description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.description(::core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.text(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            description: description::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPMetadataText as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn bandWidth(&self, plbandwidth: *mut i32) -> ::windows::core::Result<()>;
    fn recoveredPackets(&self, plrecoveredpackets: *mut i32) -> ::windows::core::Result<()>;
    fn sourceProtocol(&self, pbstrsourceprotocol: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn receivedPackets(&self, plreceivedpackets: *mut i32) -> ::windows::core::Result<()>;
    fn lostPackets(&self, pllostpackets: *mut i32) -> ::windows::core::Result<()>;
    fn receptionQuality(&self, plreceptionquality: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingCount(&self, plbufferingcount: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingProgress(&self, plbufferingprogress: *mut i32) -> ::windows::core::Result<()>;
    fn bufferingTime(&self, plbufferingtime: *mut i32) -> ::windows::core::Result<()>;
    fn SetbufferingTime(&self, lbufferingtime: i32) -> ::windows::core::Result<()>;
    fn frameRate(&self, plframerate: *mut i32) -> ::windows::core::Result<()>;
    fn maxBitRate(&self, plbitrate: *mut i32) -> ::windows::core::Result<()>;
    fn bitRate(&self, plbitrate: *mut i32) -> ::windows::core::Result<()>;
    fn getProxySettings(&self, bstrprotocol: &::windows::core::BSTR, plproxysetting: *mut i32) -> ::windows::core::Result<()>;
    fn setProxySettings(&self, bstrprotocol: &::windows::core::BSTR, lproxysetting: i32) -> ::windows::core::Result<()>;
    fn getProxyName(&self, bstrprotocol: &::windows::core::BSTR, pbstrproxyname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setProxyName(&self, bstrprotocol: &::windows::core::BSTR, bstrproxyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getProxyPort(&self, bstrprotocol: &::windows::core::BSTR, lproxyport: *mut i32) -> ::windows::core::Result<()>;
    fn setProxyPort(&self, bstrprotocol: &::windows::core::BSTR, lproxyport: i32) -> ::windows::core::Result<()>;
    fn getProxyExceptionList(&self, bstrprotocol: &::windows::core::BSTR, pbstrexceptionlist: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setProxyExceptionList(&self, bstrprotocol: &::windows::core::BSTR, pbstrexceptionlist: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getProxyBypassForLocal(&self, bstrprotocol: &::windows::core::BSTR, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn setProxyBypassForLocal(&self, bstrprotocol: &::windows::core::BSTR, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn maxBandwidth(&self, lmaxbandwidth: *mut i32) -> ::windows::core::Result<()>;
    fn SetmaxBandwidth(&self, lmaxbandwidth: i32) -> ::windows::core::Result<()>;
    fn downloadProgress(&self, pldownloadprogress: *mut i32) -> ::windows::core::Result<()>;
    fn encodedFrameRate(&self, plframerate: *mut i32) -> ::windows::core::Result<()>;
    fn framesSkipped(&self, plframes: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPNetwork {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>() -> IWMPNetwork_Vtbl {
        unsafe extern "system" fn bandWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.bandWidth(::core::mem::transmute_copy(&plbandwidth)).into()
        }
        unsafe extern "system" fn recoveredPackets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.recoveredPackets(::core::mem::transmute_copy(&plrecoveredpackets)).into()
        }
        unsafe extern "system" fn sourceProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.sourceProtocol(::core::mem::transmute_copy(&pbstrsourceprotocol)).into()
        }
        unsafe extern "system" fn receivedPackets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.receivedPackets(::core::mem::transmute_copy(&plreceivedpackets)).into()
        }
        unsafe extern "system" fn lostPackets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.lostPackets(::core::mem::transmute_copy(&pllostpackets)).into()
        }
        unsafe extern "system" fn receptionQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.receptionQuality(::core::mem::transmute_copy(&plreceptionquality)).into()
        }
        unsafe extern "system" fn bufferingCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.bufferingCount(::core::mem::transmute_copy(&plbufferingcount)).into()
        }
        unsafe extern "system" fn bufferingProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.bufferingProgress(::core::mem::transmute_copy(&plbufferingprogress)).into()
        }
        unsafe extern "system" fn bufferingTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.bufferingTime(::core::mem::transmute_copy(&plbufferingtime)).into()
        }
        unsafe extern "system" fn SetbufferingTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbufferingTime(::core::mem::transmute_copy(&lbufferingtime)).into()
        }
        unsafe extern "system" fn frameRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.frameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn maxBitRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.maxBitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn bitRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.bitRate(::core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn getProxySettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, plproxysetting: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getProxySettings(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&plproxysetting)).into()
        }
        unsafe extern "system" fn setProxySettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, lproxysetting: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxySettings(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxysetting)).into()
        }
        unsafe extern "system" fn getProxyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrproxyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getProxyName(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pbstrproxyname)).into()
        }
        unsafe extern "system" fn setProxyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrproxyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxyName(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrproxyname)).into()
        }
        unsafe extern "system" fn getProxyPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, lproxyport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getProxyPort(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn setProxyPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, lproxyport: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxyPort(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn getProxyExceptionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrexceptionlist: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getProxyExceptionList(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn setProxyExceptionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrexceptionlist: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxyExceptionList(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn getProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getProxyBypassForLocal(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pfbypassforlocal)).into()
        }
        unsafe extern "system" fn setProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxyBypassForLocal(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn maxBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.maxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn SetmaxBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetmaxBandwidth(::core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn downloadProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.downloadProgress(::core::mem::transmute_copy(&pldownloadprogress)).into()
        }
        unsafe extern "system" fn encodedFrameRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.encodedFrameRate(::core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn framesSkipped<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.framesSkipped(::core::mem::transmute_copy(&plframes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPNetwork as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstate_Impl: Sized {
    fn GetDesiredSize(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn SetRects(&self, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetRects(&self, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetWindowless(&self, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetWindowless(&self, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetFullScreen(&self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFullScreen(&self, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPNodeRealEstate {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>() -> IWMPNodeRealEstate_Vtbl {
        unsafe extern "system" fn GetDesiredSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesiredSize(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn SetRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn GetRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRects(::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn SetWindowless<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowless(::core::mem::transmute_copy(&fwindowless)).into()
        }
        unsafe extern "system" fn GetWindowless<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWindowless(::core::mem::transmute_copy(&pfwindowless)).into()
        }
        unsafe extern "system" fn SetFullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFullScreen(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn GetFullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFullScreen(::core::mem::transmute_copy(&pffullscreen)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPNodeRealEstate as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstateHost_Impl: Sized {
    fn OnDesiredSizeChange(&self, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn OnFullScreenTransition(&self, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPNodeRealEstateHost {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeRealEstateHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>() -> IWMPNodeRealEstateHost_Vtbl {
        unsafe extern "system" fn OnDesiredSizeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDesiredSizeChange(::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn OnFullScreenTransition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFullScreenTransition(::core::mem::transmute_copy(&ffullscreen)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDesiredSizeChange: OnDesiredSizeChange::<Identity, Impl, OFFSET>,
            OnFullScreenTransition: OnFullScreenTransition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeRealEstateHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPNodeWindowed_Impl: Sized {
    fn SetOwnerWindow(&self, hwnd: isize) -> ::windows::core::Result<()>;
    fn GetOwnerWindow(&self, phwnd: *mut isize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPNodeWindowed {}
impl IWMPNodeWindowed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>() -> IWMPNodeWindowed_Vtbl {
        unsafe extern "system" fn SetOwnerWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerWindow(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetOwnerWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOwnerWindow(::core::mem::transmute_copy(&phwnd)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOwnerWindow: SetOwnerWindow::<Identity, Impl, OFFSET>,
            GetOwnerWindow: GetOwnerWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowed as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowedHost_Impl: Sized {
    fn OnWindowMessageFromRenderer(&self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPNodeWindowedHost {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowedHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: isize>() -> IWMPNodeWindowedHost_Vtbl {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowMessageFromRenderer(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWindowMessageFromRenderer: OnWindowMessageFromRenderer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowedHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowless_Impl: Sized + IWMPWindowMessageSink_Impl {
    fn OnDraw(&self, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPNodeWindowless {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowless_Impl, const OFFSET: isize>() -> IWMPNodeWindowless_Vtbl {
        unsafe extern "system" fn OnDraw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDraw(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prcdraw)).into()
        }
        Self { base__: IWMPWindowMessageSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnDraw: OnDraw::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowless as ::windows::core::ComInterface>::IID || iid == &<IWMPWindowMessageSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessHost_Impl: Sized {
    fn InvalidateRect(&self, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPNodeWindowlessHost {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPNodeWindowlessHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: isize>() -> IWMPNodeWindowlessHost_Vtbl {
        unsafe extern "system" fn InvalidateRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateRect(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&ferase)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvalidateRect: InvalidateRect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPNodeWindowlessHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer_Impl: Sized + IWMPCore_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlayer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>() -> IWMPPlayer_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuiMode(::core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPPlayer as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer2_Impl: Sized + IWMPCore_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlayer2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>() -> IWMPPlayer2_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuiMode(::core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPPlayer2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer3_Impl: Sized + IWMPCore2_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlayer3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>() -> IWMPPlayer3_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuiMode(::core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IWMPCore2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPPlayer3 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID || iid == &<IWMPCore2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayer4_Impl: Sized + IWMPCore3_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetuiMode(&self, bstrmode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn isRemote(&self, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn playerApplication(&self) -> ::windows::core::Result<IWMPPlayerApplication>;
    fn openPlayer(&self, bstrurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlayer4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>() -> IWMPPlayer4_Vtbl {
        unsafe extern "system" fn enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enabled(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setenabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fullScreen(::core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetfullScreen(::core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enableContextMenu(::core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetenableContextMenu(::core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuiMode(::core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.uiMode(::core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stretchToFit(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetstretchToFit(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.windowlessVideo(::core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetwindowlessVideo(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn isRemote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isRemote(::core::mem::transmute_copy(&pvarfisremote)).into()
        }
        unsafe extern "system" fn playerApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.playerApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpplayerapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openPlayer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.openPlayer(::core::mem::transmute(&bstrurl)).into()
        }
        Self {
            base__: IWMPCore3_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPPlayer4 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPCore as ::windows::core::ComInterface>::IID || iid == &<IWMPCore2 as ::windows::core::ComInterface>::IID || iid == &<IWMPCore3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlayerApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn switchToPlayerApplication(&self) -> ::windows::core::Result<()>;
    fn switchToControl(&self) -> ::windows::core::Result<()>;
    fn playerDocked(&self, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn hasDisplay(&self, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlayerApplication {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlayerApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>() -> IWMPPlayerApplication_Vtbl {
        unsafe extern "system" fn switchToPlayerApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.switchToPlayerApplication().into()
        }
        unsafe extern "system" fn switchToControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.switchToControl().into()
        }
        unsafe extern "system" fn playerDocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.playerDocked(::core::mem::transmute_copy(&pbplayerdocked)).into()
        }
        unsafe extern "system" fn hasDisplay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hasDisplay(::core::mem::transmute_copy(&pbhasdisplay)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            switchToPlayerApplication: switchToPlayerApplication::<Identity, Impl, OFFSET>,
            switchToControl: switchToControl::<Identity, Impl, OFFSET>,
            playerDocked: playerDocked::<Identity, Impl, OFFSET>,
            hasDisplay: hasDisplay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerApplication as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPPlayerServices_Impl: Sized {
    fn activateUIPlugin(&self, bstrplugin: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPane(&self, bstrtaskpane: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setTaskPaneURL(&self, bstrtaskpane: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPPlayerServices {}
impl IWMPPlayerServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>() -> IWMPPlayerServices_Vtbl {
        unsafe extern "system" fn activateUIPlugin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrplugin: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.activateUIPlugin(::core::mem::transmute(&bstrplugin)).into()
        }
        unsafe extern "system" fn setTaskPane<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setTaskPane(::core::mem::transmute(&bstrtaskpane)).into()
        }
        unsafe extern "system" fn setTaskPaneURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setTaskPaneURL(::core::mem::transmute(&bstrtaskpane), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrfriendlyname)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            activateUIPlugin: activateUIPlugin::<Identity, Impl, OFFSET>,
            setTaskPane: setTaskPane::<Identity, Impl, OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPPlayerServices2_Impl: Sized + IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(&self, bstrpriority: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPPlayerServices2 {}
impl IWMPPlayerServices2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices2_Impl, const OFFSET: isize>() -> IWMPPlayerServices2_Vtbl {
        unsafe extern "system" fn setBackgroundProcessingPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlayerServices2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpriority: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setBackgroundProcessingPriority(::core::mem::transmute(&bstrpriority)).into()
        }
        Self {
            base__: IWMPPlayerServices_Vtbl::new::<Identity, Impl, OFFSET>(),
            setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as ::windows::core::ComInterface>::IID || iid == &<IWMPPlayerServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylist_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn name(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Setname(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn attributeCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn get_attributeName(&self, lindex: i32, pbstrattributename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_item(&self, lindex: i32) -> ::windows::core::Result<IWMPMedia>;
    fn getItemInfo(&self, bstrname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setItemInfo(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_isIdentical(&self, piwmpplaylist: ::core::option::Option<&IWMPPlaylist>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn clear(&self) -> ::windows::core::Result<()>;
    fn insertItem(&self, lindex: i32, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn appendItem(&self, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn removeItem(&self, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows::core::Result<()>;
    fn moveItem(&self, lindexold: i32, lindexnew: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlaylist {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>() -> IWMPPlaylist_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.name(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setname(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn get_attributeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_attributeName(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrattributename)).into()
        }
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmedia, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setItemInfo(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn get_isIdentical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpplaylist: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_isIdentical(::windows::core::from_raw_borrowed(&piwmpplaylist), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.clear().into()
        }
        unsafe extern "system" fn insertItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.insertItem(::core::mem::transmute_copy(&lindex), ::windows::core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn appendItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.appendItem(::windows::core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn removeItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeItem(::windows::core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn moveItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.moveItem(::core::mem::transmute_copy(&lindexold), ::core::mem::transmute_copy(&lindexnew)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Setname: Setname::<Identity, Impl, OFFSET>,
            attributeCount: attributeCount::<Identity, Impl, OFFSET>,
            get_attributeName: get_attributeName::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            setItemInfo: setItemInfo::<Identity, Impl, OFFSET>,
            get_isIdentical: get_isIdentical::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            insertItem: insertItem::<Identity, Impl, OFFSET>,
            appendItem: appendItem::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            moveItem: moveItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylist as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistArray_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&self, lindex: i32) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlaylistArray {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>() -> IWMPPlaylistArray_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPPlaylistCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn newPlaylist(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylist>;
    fn getAll(&self) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn getByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IWMPPlaylistArray>;
    fn remove(&self, pitem: ::core::option::Option<&IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn setDeleted(&self, pitem: ::core::option::Option<&IWMPPlaylist>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn isDeleted(&self, pitem: ::core::option::Option<&IWMPPlaylist>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn importPlaylist(&self, pitem: ::core::option::Option<&IWMPPlaylist>) -> ::windows::core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPPlaylistCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPPlaylistCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>() -> IWMPPlaylistCollection_Vtbl {
        unsafe extern "system" fn newPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.newPlaylist(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAll() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistarray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getByName(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistarray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remove(::windows::core::from_raw_borrowed(&pitem)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setDeleted(::windows::core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isDeleted(::windows::core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        unsafe extern "system" fn importPlaylist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, ppimporteditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.importPlaylist(::windows::core::from_raw_borrowed(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimporteditem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMPPlaylistCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPPlugin_Impl: Sized {
    fn Init(&self, dwplaybackcontext: usize) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
    fn GetID(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetCaps(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn AdviseWMPServices(&self, pwmpservices: ::core::option::Option<&IWMPServices>) -> ::windows::core::Result<()>;
    fn UnAdviseWMPServices(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPPlugin {}
impl IWMPPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>() -> IWMPPlugin_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&dwplaybackcontext)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetID(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn AdviseWMPServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpservices: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseWMPServices(::windows::core::from_raw_borrowed(&pwmpservices)).into()
        }
        unsafe extern "system" fn UnAdviseWMPServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnAdviseWMPServices().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            AdviseWMPServices: AdviseWMPServices::<Identity, Impl, OFFSET>,
            UnAdviseWMPServices: UnAdviseWMPServices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPlugin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPluginEnable_Impl: Sized {
    fn SetEnable(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnable(&self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPPluginEnable {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPPluginEnable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>() -> IWMPPluginEnable_Vtbl {
        unsafe extern "system" fn SetEnable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetEnable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEnable(::core::mem::transmute_copy(&pfenable)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEnable: SetEnable::<Identity, Impl, OFFSET>,
            GetEnable: GetEnable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPPluginEnable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWMPPluginUI_Impl: Sized {
    fn SetCore(&self, pcore: ::core::option::Option<&IWMPCore>) -> ::windows::core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Destroy(&self) -> ::windows::core::Result<()>;
    fn DisplayPropertyPage(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetProperty(&self, pwszname: &::windows::core::PCWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetProperty(&self, pwszname: &::windows::core::PCWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TranslateAccelerator(&self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IWMPPluginUI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWMPPluginUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>() -> IWMPPluginUI_Vtbl {
        unsafe extern "system" fn SetCore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCore(::windows::core::from_raw_borrowed(&pcore)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&phwndwindow)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayPropertyPage(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pvarproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pvarproperty: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TranslateAccelerator(::core::mem::transmute_copy(&lpmsg)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPPluginUI as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPQuery_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn addCondition(&self, bstrattribute: &::windows::core::BSTR, bstroperator: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn beginNextGroup(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPQuery {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: isize>() -> IWMPQuery_Vtbl {
        unsafe extern "system" fn addCondition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstroperator: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addCondition(::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstroperator), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn beginNextGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.beginNextGroup().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            addCondition: addCondition::<Identity, Impl, OFFSET>,
            beginNextGroup: beginNextGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPQuery as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPRemoteMediaServices_Impl: Sized {
    fn GetServiceType(&self, pbstrtype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetApplicationName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetScriptableObject(&self, pbstrname: *mut ::windows::core::BSTR, ppdispatch: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetCustomUIMode(&self, pbstrfile: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMPRemoteMediaServices {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPRemoteMediaServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>() -> IWMPRemoteMediaServices_Vtbl {
        unsafe extern "system" fn GetServiceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetServiceType(::core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetScriptableObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptableObject(::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&ppdispatch)).into()
        }
        unsafe extern "system" fn GetCustomUIMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCustomUIMode(::core::mem::transmute_copy(&pbstrfile)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServiceType: GetServiceType::<Identity, Impl, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, Impl, OFFSET>,
            GetScriptableObject: GetScriptableObject::<Identity, Impl, OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPRemoteMediaServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPRenderConfig_Impl: Sized {
    fn SetinProcOnly(&self, finproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn inProcOnly(&self, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPRenderConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>() -> IWMPRenderConfig_Vtbl {
        unsafe extern "system" fn SetinProcOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetinProcOnly(::core::mem::transmute_copy(&finproc)).into()
        }
        unsafe extern "system" fn inProcOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.inProcOnly(::core::mem::transmute_copy(&pfinproc)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetinProcOnly: SetinProcOnly::<Identity, Impl, OFFSET>,
            inProcOnly: inProcOnly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPRenderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPServices_Impl: Sized {
    fn GetStreamTime(&self, prt: *mut i64) -> ::windows::core::Result<()>;
    fn GetStreamState(&self, pstate: *mut WMPServices_StreamState) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPServices {}
impl IWMPServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: isize>() -> IWMPServices_Vtbl {
        unsafe extern "system" fn GetStreamTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamTime(::core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetStreamState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamState(::core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamTime: GetStreamTime::<Identity, Impl, OFFSET>,
            GetStreamState: GetStreamState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &::windows::core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn autoStart(&self, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetautoStart(&self, fautostart: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn baseURL(&self, pbstrbaseurl: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetbaseURL(&self, bstrbaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn defaultFrame(&self, pbstrdefaultframe: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetdefaultFrame(&self, bstrdefaultframe: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn invokeURLs(&self, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetinvokeURLs(&self, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn mute(&self, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setmute(&self, fmute: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn playCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetplayCount(&self, lcount: i32) -> ::windows::core::Result<()>;
    fn rate(&self, pdrate: *mut f64) -> ::windows::core::Result<()>;
    fn Setrate(&self, drate: f64) -> ::windows::core::Result<()>;
    fn balance(&self, plbalance: *mut i32) -> ::windows::core::Result<()>;
    fn Setbalance(&self, lbalance: i32) -> ::windows::core::Result<()>;
    fn volume(&self, plvolume: *mut i32) -> ::windows::core::Result<()>;
    fn Setvolume(&self, lvolume: i32) -> ::windows::core::Result<()>;
    fn getMode(&self, bstrmode: &::windows::core::BSTR, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn setMode(&self, bstrmode: &::windows::core::BSTR, varfmode: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn enableErrorDialogs(&self, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetenableErrorDialogs(&self, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>() -> IWMPSettings_Vtbl {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows::core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_isAvailable(::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn autoStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.autoStart(::core::mem::transmute_copy(&pfautostart)).into()
        }
        unsafe extern "system" fn SetautoStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fautostart: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetautoStart(::core::mem::transmute_copy(&fautostart)).into()
        }
        unsafe extern "system" fn baseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.baseURL(::core::mem::transmute_copy(&pbstrbaseurl)).into()
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbaseURL(::core::mem::transmute(&bstrbaseurl)).into()
        }
        unsafe extern "system" fn defaultFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.defaultFrame(::core::mem::transmute_copy(&pbstrdefaultframe)).into()
        }
        unsafe extern "system" fn SetdefaultFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdefaultFrame(::core::mem::transmute(&bstrdefaultframe)).into()
        }
        unsafe extern "system" fn invokeURLs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.invokeURLs(::core::mem::transmute_copy(&pfinvokeurls)).into()
        }
        unsafe extern "system" fn SetinvokeURLs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetinvokeURLs(::core::mem::transmute_copy(&finvokeurls)).into()
        }
        unsafe extern "system" fn mute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.mute(::core::mem::transmute_copy(&pfmute)).into()
        }
        unsafe extern "system" fn Setmute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmute: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setmute(::core::mem::transmute_copy(&fmute)).into()
        }
        unsafe extern "system" fn playCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.playCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn SetplayCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetplayCount(::core::mem::transmute_copy(&lcount)).into()
        }
        unsafe extern "system" fn rate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.rate(::core::mem::transmute_copy(&pdrate)).into()
        }
        unsafe extern "system" fn Setrate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setrate(::core::mem::transmute_copy(&drate)).into()
        }
        unsafe extern "system" fn balance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.balance(::core::mem::transmute_copy(&plbalance)).into()
        }
        unsafe extern "system" fn Setbalance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setbalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn volume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.volume(::core::mem::transmute_copy(&plvolume)).into()
        }
        unsafe extern "system" fn Setvolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setvolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn getMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getMode(::core::mem::transmute(&bstrmode), ::core::mem::transmute_copy(&pvarfmode)).into()
        }
        unsafe extern "system" fn setMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows::core::BSTR>, varfmode: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setMode(::core::mem::transmute(&bstrmode), ::core::mem::transmute_copy(&varfmode)).into()
        }
        unsafe extern "system" fn enableErrorDialogs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.enableErrorDialogs(::core::mem::transmute_copy(&pfenableerrordialogs)).into()
        }
        unsafe extern "system" fn SetenableErrorDialogs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetenableErrorDialogs(::core::mem::transmute_copy(&fenableerrordialogs)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
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
        iid == &<IWMPSettings as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPSettings2_Impl: Sized + IWMPSettings_Impl {
    fn defaultAudioLanguage(&self, pllangid: *mut i32) -> ::windows::core::Result<()>;
    fn mediaAccessRights(&self, pbstrrights: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn requestMediaAccessRights(&self, bstrdesiredaccess: &::windows::core::BSTR, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPSettings2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: isize>() -> IWMPSettings2_Vtbl {
        unsafe extern "system" fn defaultAudioLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.defaultAudioLanguage(::core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn mediaAccessRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.mediaAccessRights(::core::mem::transmute_copy(&pbstrrights)).into()
        }
        unsafe extern "system" fn requestMediaAccessRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.requestMediaAccessRights(::core::mem::transmute(&bstrdesiredaccess), ::core::mem::transmute_copy(&pvbaccepted)).into()
        }
        Self {
            base__: IWMPSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            defaultAudioLanguage: defaultAudioLanguage::<Identity, Impl, OFFSET>,
            mediaAccessRights: mediaAccessRights::<Identity, Impl, OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSettings2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPSettings as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPSkinManager_Impl: Sized {
    fn SetVisualStyle(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPSkinManager {}
impl IWMPSkinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSkinManager_Impl, const OFFSET: isize>() -> IWMPSkinManager_Vtbl {
        unsafe extern "system" fn SetVisualStyle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSkinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVisualStyle(::core::mem::transmute(&bstrpath)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVisualStyle: SetVisualStyle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSkinManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn item(&self, lindex: i32, pbstrstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPStringCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: isize>() -> IWMPStringCollection_Vtbl {
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.item(::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMPStringCollection2_Impl: Sized + IWMPStringCollection_Impl {
    fn isIdentical(&self, piwmpstringcollection2: ::core::option::Option<&IWMPStringCollection2>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getItemInfo(&self, lcollectionindex: i32, bstritemname: &::windows::core::BSTR, pbstrvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeCountByType(&self, lcollectionindex: i32, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfoByType(&self, lcollectionindex: i32, bstrtype: &::windows::core::BSTR, bstrlanguage: &::windows::core::BSTR, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMPStringCollection2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMPStringCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>() -> IWMPStringCollection2_Vtbl {
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isIdentical(::windows::core::from_raw_borrowed(&piwmpstringcollection2), ::core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAttributeCountByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfoByType(::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&lattributeindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: IWMPStringCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPStringCollection2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWMPStringCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService_Impl: Sized {
    fn allowPlay(&self, hwnd: super::super::Foundation::HWND, pmedia: ::core::option::Option<&IWMPMedia>, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowCDBurn(&self, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<&IWMPPlaylist>, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn allowPDATransfer(&self, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<&IWMPPlaylist>, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn startBackgroundProcessing(&self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPSubscriptionService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>() -> IWMPSubscriptionService_Vtbl {
        unsafe extern "system" fn allowPlay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: *mut ::core::ffi::c_void, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.allowPlay(::core::mem::transmute_copy(&hwnd), ::windows::core::from_raw_borrowed(&pmedia), ::core::mem::transmute_copy(&pfallowplay)).into()
        }
        unsafe extern "system" fn allowCDBurn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.allowCDBurn(::core::mem::transmute_copy(&hwnd), ::windows::core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pfallowburn)).into()
        }
        unsafe extern "system" fn allowPDATransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.allowPDATransfer(::core::mem::transmute_copy(&hwnd), ::windows::core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pfallowtransfer)).into()
        }
        unsafe extern "system" fn startBackgroundProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startBackgroundProcessing(::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            allowPlay: allowPlay::<Identity, Impl, OFFSET>,
            allowCDBurn: allowCDBurn::<Identity, Impl, OFFSET>,
            allowPDATransfer: allowPDATransfer::<Identity, Impl, OFFSET>,
            startBackgroundProcessing: startBackgroundProcessing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionService as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService2_Impl: Sized + IWMPSubscriptionService_Impl {
    fn stopBackgroundProcessing(&self) -> ::windows::core::Result<()>;
    fn serviceEvent(&self, event: WMPSubscriptionServiceEvent) -> ::windows::core::Result<()>;
    fn deviceAvailable(&self, bstrdevicename: &::windows::core::BSTR, pcb: ::core::option::Option<&IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
    fn prepareForSync(&self, bstrfilename: &::windows::core::BSTR, bstrdevicename: &::windows::core::BSTR, pcb: ::core::option::Option<&IWMPSubscriptionServiceCallback>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPSubscriptionService2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSubscriptionService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>() -> IWMPSubscriptionService2_Vtbl {
        unsafe extern "system" fn stopBackgroundProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stopBackgroundProcessing().into()
        }
        unsafe extern "system" fn serviceEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.serviceEvent(::core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn deviceAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deviceAvailable(::core::mem::transmute(&bstrdevicename), ::windows::core::from_raw_borrowed(&pcb)).into()
        }
        unsafe extern "system" fn prepareForSync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.prepareForSync(::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrdevicename), ::windows::core::from_raw_borrowed(&pcb)).into()
        }
        Self {
            base__: IWMPSubscriptionService_Vtbl::new::<Identity, Impl, OFFSET>(),
            stopBackgroundProcessing: stopBackgroundProcessing::<Identity, Impl, OFFSET>,
            serviceEvent: serviceEvent::<Identity, Impl, OFFSET>,
            deviceAvailable: deviceAvailable::<Identity, Impl, OFFSET>,
            prepareForSync: prepareForSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionService2 as ::windows::core::ComInterface>::IID || iid == &<IWMPSubscriptionService as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPSubscriptionServiceCallback_Impl: Sized {
    fn onComplete(&self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPSubscriptionServiceCallback {}
impl IWMPSubscriptionServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: isize>() -> IWMPSubscriptionServiceCallback_Vtbl {
        unsafe extern "system" fn onComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onComplete(::core::mem::transmute_copy(&hrresult)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSubscriptionServiceCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice_Impl: Sized {
    fn friendlyName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetfriendlyName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn deviceName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn deviceId(&self, pbstrdeviceid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn partnershipIndex(&self, plindex: *mut i32) -> ::windows::core::Result<()>;
    fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn status(&self, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::Result<()>;
    fn syncState(&self, pwmpss: *mut WMPSyncState) -> ::windows::core::Result<()>;
    fn progress(&self, plprogress: *mut i32) -> ::windows::core::Result<()>;
    fn getItemInfo(&self, bstritemname: &::windows::core::BSTR, pbstrval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn createPartnership(&self, vbshowui: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn deletePartnership(&self) -> ::windows::core::Result<()>;
    fn start(&self) -> ::windows::core::Result<()>;
    fn stop(&self) -> ::windows::core::Result<()>;
    fn showSettings(&self) -> ::windows::core::Result<()>;
    fn isIdentical(&self, pdevice: ::core::option::Option<&IWMPSyncDevice>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPSyncDevice {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>() -> IWMPSyncDevice_Vtbl {
        unsafe extern "system" fn friendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.friendlyName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn SetfriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetfriendlyName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn deviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deviceName(::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn deviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deviceId(::core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn partnershipIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.partnershipIndex(::core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn connected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.connected(::core::mem::transmute_copy(&pvbconnected)).into()
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.status(::core::mem::transmute_copy(&pwmpds)).into()
        }
        unsafe extern "system" fn syncState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.syncState(::core::mem::transmute_copy(&pwmpss)).into()
        }
        unsafe extern "system" fn progress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.progress(::core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn createPartnership<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbshowui: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.createPartnership(::core::mem::transmute_copy(&vbshowui)).into()
        }
        unsafe extern "system" fn deletePartnership<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deletePartnership().into()
        }
        unsafe extern "system" fn start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.start().into()
        }
        unsafe extern "system" fn stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.stop().into()
        }
        unsafe extern "system" fn showSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.showSettings().into()
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.isIdentical(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMPSyncDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice2_Impl: Sized + IWMPSyncDevice_Impl {
    fn setItemInfo(&self, bstritemname: &::windows::core::BSTR, bstrval: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPSyncDevice2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPSyncDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice2_Impl, const OFFSET: isize>() -> IWMPSyncDevice2_Vtbl {
        unsafe extern "system" fn setItemInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setItemInfo(::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrval)).into()
        }
        Self { base__: IWMPSyncDevice_Vtbl::new::<Identity, Impl, OFFSET>(), setItemInfo: setItemInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as ::windows::core::ComInterface>::IID || iid == &<IWMPSyncDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSyncDevice3_Impl: Sized + IWMPSyncDevice2_Impl {
    fn estimateSyncSize(&self, pnonruleplaylist: ::core::option::Option<&IWMPPlaylist>, prulesplaylist: ::core::option::Option<&IWMPPlaylist>) -> ::windows::core::Result<()>;
    fn cancelEstimation(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMPSyncDevice3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMPSyncDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>() -> IWMPSyncDevice3_Vtbl {
        unsafe extern "system" fn estimateSyncSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: *mut ::core::ffi::c_void, prulesplaylist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.estimateSyncSize(::windows::core::from_raw_borrowed(&pnonruleplaylist), ::windows::core::from_raw_borrowed(&prulesplaylist)).into()
        }
        unsafe extern "system" fn cancelEstimation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.cancelEstimation().into()
        }
        Self {
            base__: IWMPSyncDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Identity, Impl, OFFSET>,
            cancelEstimation: cancelEstimation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as ::windows::core::ComInterface>::IID || iid == &<IWMPSyncDevice as ::windows::core::ComInterface>::IID || iid == &<IWMPSyncDevice2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPSyncServices_Impl: Sized {
    fn deviceCount(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn getDevice(&self, lindex: i32) -> ::windows::core::Result<IWMPSyncDevice>;
}
impl ::windows::core::RuntimeName for IWMPSyncServices {}
impl IWMPSyncServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: isize>() -> IWMPSyncServices_Vtbl {
        unsafe extern "system" fn deviceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deviceCount(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDevice(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            deviceCount: deviceCount::<Identity, Impl, OFFSET>,
            getDevice: getDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPSyncServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPTranscodePolicy_Impl: Sized {
    fn allowTranscode(&self, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPTranscodePolicy {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPTranscodePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPTranscodePolicy_Impl, const OFFSET: isize>() -> IWMPTranscodePolicy_Vtbl {
        unsafe extern "system" fn allowTranscode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPTranscodePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.allowTranscode(::core::mem::transmute_copy(&pvballow)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), allowTranscode: allowTranscode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPTranscodePolicy as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IWMPUserEventSink_Impl: Sized {
    fn NotifyUserEvent(&self, eventcode: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPUserEventSink {}
impl IWMPUserEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPUserEventSink_Impl, const OFFSET: isize>() -> IWMPUserEventSink_Vtbl {
        unsafe extern "system" fn NotifyUserEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPUserEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyUserEvent(::core::mem::transmute_copy(&eventcode)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyUserEvent: NotifyUserEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPUserEventSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Media_MediaFoundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfig_Impl: Sized {
    fn SetpresenterActivate(&self, pactivate: ::core::option::Option<&super::MediaFoundation::IMFActivate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl ::windows::core::RuntimeName for IWMPVideoRenderConfig {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IWMPVideoRenderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: isize>() -> IWMPVideoRenderConfig_Vtbl {
        unsafe extern "system" fn SetpresenterActivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetpresenterActivate(::windows::core::from_raw_borrowed(&pactivate)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetpresenterActivate: SetpresenterActivate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPVideoRenderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPWindowMessageSink_Impl: Sized {
    fn OnWindowMessage(&self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMPWindowMessageSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWMPWindowMessageSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPWindowMessageSink_Impl, const OFFSET: isize>() -> IWMPWindowMessageSink_Vtbl {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPWindowMessageSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowMessage(::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPWindowMessageSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed_Impl: Sized {
    fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Rename(&self, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetUrl(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn LocalId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Move(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Parent(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LastWriteTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Download(&self) -> ::windows::core::Result<()>;
    fn AsyncDownload(&self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&self) -> ::windows::core::Result<()>;
    fn SyncSetting(&self) -> ::windows::core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<u32>;
    fn SetInterval(&self, uiinterval: u32) -> ::windows::core::Result<()>;
    fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn LocalEnclosurePath(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Items(&self) -> ::windows::core::Result<IXFeedsEnum>;
    fn GetItem(&self, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn MarkAllItemsRead(&self) -> ::windows::core::Result<()>;
    fn MaxItemCount(&self) -> ::windows::core::Result<u32>;
    fn SetMaxItemCount(&self, uimaxitemcount: u32) -> ::windows::core::Result<()>;
    fn DownloadEnclosuresAutomatically(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetDownloadEnclosuresAutomatically(&self, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Image(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn LastBuildDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Ttl(&self) -> ::windows::core::Result<u32>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Copyright(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UnreadItemCount(&self) -> ::windows::core::Result<u32>;
    fn ItemCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXFeed {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>() -> IXFeed_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Xml(::core::mem::transmute_copy(&uiitemcount), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUrl(::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWriteTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastwritetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download().into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncDownload().into()
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SyncSetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfss, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncSetting(::core::mem::transmute_copy(&fss)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Interval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiinterval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastdownloadtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalEnclosurePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Items() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItem(::core::mem::transmute_copy(&uiid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarkAllItemsRead().into()
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puimaxitemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxItemCount(::core::mem::transmute_copy(&uimaxitemcount)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadEnclosuresAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdownloadenclosuresautomatically, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDownloadEnclosuresAutomatically(::core::mem::transmute_copy(&bdownloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfde, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Merge(::windows::core::from_raw_borrowed(&pstream), ::core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztitle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszhomepage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Image() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastBuildDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastbuilddate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstpubdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ttl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puittl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlanguage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Copyright() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcopyright, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbislist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiunreaditemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiitemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeed as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed2_Impl: Sized + IXFeed_Impl {
    fn GetItemByEffectiveId(&self, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LastItemDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Username(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetCredentials(&self, pszusername: &::windows::core::PCWSTR, pszpassword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ClearCredentials(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXFeed2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeed2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>() -> IXFeed2_Vtbl {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItemByEffectiveId(::core::mem::transmute_copy(&uieffectiveid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastItemDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastitemdownloadtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszusername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Username() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Password() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpassword, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearCredentials().into()
        }
        Self {
            base__: IXFeed_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeed2 as ::windows::core::ComInterface>::IID || iid == &<IXFeed as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IXFeedEnclosure_Impl: Sized {
    fn Url(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn AsyncDownload(&self) -> ::windows::core::Result<()>;
    fn CancelAsyncDownload(&self) -> ::windows::core::Result<()>;
    fn DownloadStatus(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> ::windows::core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Parent(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn DownloadMimeType(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn RemoveFile(&self) -> ::windows::core::Result<()>;
    fn SetFile(&self, pszdownloadurl: &::windows::core::PCWSTR, pszdownloadfilepath: &::windows::core::PCWSTR, pszdownloadmimetype: &::windows::core::PCWSTR, pszenclosurefilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXFeedEnclosure {}
impl IXFeedEnclosure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>() -> IXFeedEnclosure_Vtbl {
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszmimetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puilength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncDownload().into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncDownload().into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfde, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadMimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszmimetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFile().into()
        }
        unsafe extern "system" fn SetFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdownloadurl: ::windows::core::PCWSTR, pszdownloadfilepath: ::windows::core::PCWSTR, pszdownloadmimetype: ::windows::core::PCWSTR, pszenclosurefilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFile(::core::mem::transmute(&pszdownloadurl), ::core::mem::transmute(&pszdownloadfilepath), ::core::mem::transmute(&pszdownloadmimetype), ::core::mem::transmute(&pszenclosurefilename)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedEnclosure as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IXFeedEvents_Impl: Sized {
    fn Error(&self) -> ::windows::core::Result<()>;
    fn FeedDeleted(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedMoved(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&self, pszpath: &::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&self, pszpath: &::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXFeedEvents {}
impl IXFeedEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>() -> IXFeedEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error().into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDeleted(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedRenamed(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedUrlChanged(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMoved(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloading(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloadCompleted(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedItemCountChanged(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedFolder_Impl: Sized {
    fn Feeds(&self) -> ::windows::core::Result<IXFeedsEnum>;
    fn Subfolders(&self) -> ::windows::core::Result<IXFeedsEnum>;
    fn CreateFeed(&self, pszname: &::windows::core::PCWSTR, pszurl: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateSubfolder(&self, pszname: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExistsFeed(&self, pszname: &::windows::core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ExistsSubfolder(&self, pszname: &::windows::core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFeed(&self, pszname: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSubfolder(&self, pszname: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Rename(&self, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Move(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Parent(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsRoot(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn TotalUnreadItemCount(&self) -> ::windows::core::Result<u32>;
    fn TotalItemCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXFeedFolder {}
#[cfg(feature = "Win32_Foundation")]
impl IXFeedFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>() -> IXFeedFolder_Vtbl {
        unsafe extern "system" fn Feeds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Feeds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subfolders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pszurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateFeed(::core::mem::transmute(&pszname), ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSubfolder(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExistsFeed(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pbfeedexists)).into()
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExistsSubfolder(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pbsubfolderexists)).into()
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeed(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSubfolder(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisrootfeedfolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWatcher(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalUnreadItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puitotalunreaditemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puitotalitemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedFolder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IXFeedFolderEvents_Impl: Sized {
    fn Error(&self) -> ::windows::core::Result<()>;
    fn FolderAdded(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FolderDeleted(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FolderRenamed(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FolderMovedFrom(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FolderMovedTo(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FolderItemCountChanged(&self, pszpath: &::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
    fn FeedAdded(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedDeleted(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedRenamed(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedUrlChanged(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedMovedFrom(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedMovedTo(&self, pszpath: &::windows::core::PCWSTR, pszoldpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloading(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn FeedDownloadCompleted(&self, pszpath: &::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::Result<()>;
    fn FeedItemCountChanged(&self, pszpath: &::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXFeedFolderEvents {}
impl IXFeedFolderEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>() -> IXFeedFolderEvents_Vtbl {
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Error().into()
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderAdded(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderDeleted(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderRenamed(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderMovedFrom(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderMovedTo(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FolderItemCountChanged(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedAdded(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDeleted(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedRenamed(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedUrlChanged(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMovedFrom(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pszoldpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedMovedTo(::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloading(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedDownloadCompleted(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, feicfflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FeedItemCountChanged(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedFolderEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItem_Impl: Sized {
    fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Guid(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn PubDate(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Comments(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Enclosure(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsRead(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsRead(&self, bisread: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LocalId(&self) -> ::windows::core::Result<u32>;
    fn Parent(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn DownloadUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn LastDownloadTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Modified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXFeedItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>() -> IXFeedItem_Vtbl {
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Xml(::core::mem::transmute_copy(&fxif)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztitle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszguid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Guid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PubDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstpubdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Comments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Author() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszauthor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enclosure(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisread, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsRead(::core::mem::transmute_copy(&bisread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Parent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDownloadTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastdownloadtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Modified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstmodifiedtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItem2_Impl: Sized + IXFeedItem_Impl {
    fn EffectiveId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXFeedItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem2_Impl, const OFFSET: isize>() -> IXFeedItem2_Vtbl {
        unsafe extern "system" fn EffectiveId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EffectiveId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puieffectiveid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IXFeedItem_Vtbl::new::<Identity, Impl, OFFSET>(), EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedItem2 as ::windows::core::ComInterface>::IID || iid == &<IXFeedItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"implement\"`*"]
pub trait IXFeedsEnum_Impl: Sized {
    fn Count(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXFeedsEnum {}
impl IXFeedsEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: isize>() -> IXFeedsEnum_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puicount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Count: Count::<Identity, Impl, OFFSET>, Item: Item::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXFeedsEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedsManager_Impl: Sized {
    fn RootFolder(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsSubscribed(&self, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ExistsFeed(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFeed(&self, pszpath: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFeedByUrl(&self, pszurl: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExistsFolder(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFolder(&self, pszpath: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DeleteFeed(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DeleteFolder(&self, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn BackgroundSync(&self, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::Result<()>;
    fn BackgroundSyncStatus(&self) -> ::windows::core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultInterval(&self, uiinterval: u32) -> ::windows::core::Result<()>;
    fn AsyncSyncAll(&self) -> ::windows::core::Result<()>;
    fn Normalize(&self, pstreamin: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn ItemCountLimit(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXFeedsManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXFeedsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>() -> IXFeedsManager_Vtbl {
        unsafe extern "system" fn RootFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RootFolder(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSubscribed(::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsubscribed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsFeed(::core::mem::transmute(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbfeedexists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeed(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszurl: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeedByUrl(::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExistsFolder(::core::mem::transmute(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbfolderexists, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFolder(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFeed(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFolder(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackgroundSync(::core::mem::transmute_copy(&fbsa)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackgroundSyncStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbss, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiinterval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultInterval(::core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncSyncAll().into()
        }
        unsafe extern "system" fn Normalize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamin: *mut ::core::ffi::c_void, ppstreamout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Normalize(::windows::core::from_raw_borrowed(&pstreamin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstreamout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCountLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiitemcountlimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXFeedsManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaPlayer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _WMPOCXEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _WMPOCXEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _WMPOCXEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _WMPOCXEvents_Impl, const OFFSET: isize>() -> _WMPOCXEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_WMPOCXEvents as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
