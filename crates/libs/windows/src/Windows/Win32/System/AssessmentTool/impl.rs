#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSAT_Impl: Sized + super::Com::IDispatch_Impl + super::super::UI::Accessibility::IAccessible_Impl {
    fn SetAccessiblityData(&self, wsname: &::windows::core::PCWSTR, wsvalue: &::windows::core::PCWSTR, wsdesc: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
impl ::windows::core::RuntimeName for IAccessibleWinSAT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: isize>() -> IAccessibleWinSAT_Vtbl {
        unsafe extern "system" fn SetAccessiblityData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessibleWinSAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsname: ::windows::core::PCWSTR, wsvalue: ::windows::core::PCWSTR, wsdesc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessiblityData(::core::mem::transmute(&wsname), ::core::mem::transmute(&wsvalue), ::core::mem::transmute(&wsdesc)).into()
        }
        Self {
            base__: super::super::UI::Accessibility::IAccessible_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAccessiblityData: SetAccessiblityData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleWinSAT as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<super::super::UI::Accessibility::IAccessible as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInitiateWinSATAssessment_Impl: Sized {
    fn InitiateAssessment(&self, cmdline: &::windows::core::PCWSTR, pcallbacks: &::core::option::Option<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn InitiateFormalAssessment(&self, pcallbacks: &::core::option::Option<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CancelAssessment(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInitiateWinSATAssessment {}
#[cfg(feature = "Win32_Foundation")]
impl IInitiateWinSATAssessment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>() -> IInitiateWinSATAssessment_Vtbl {
        unsafe extern "system" fn InitiateAssessment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmdline: ::windows::core::PCWSTR, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitiateAssessment(::core::mem::transmute(&cmdline), ::core::mem::transmute(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn InitiateFormalAssessment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitiateFormalAssessment(::core::mem::transmute(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn CancelAssessment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitiateWinSATAssessment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAssessment().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitiateAssessment: InitiateAssessment::<Identity, Impl, OFFSET>,
            InitiateFormalAssessment: InitiateFormalAssessment::<Identity, Impl, OFFSET>,
            CancelAssessment: CancelAssessment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitiateWinSATAssessment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProvideWinSATAssessmentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Score(&self) -> ::windows::core::Result<f32>;
    fn Title(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IProvideWinSATAssessmentInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProvideWinSATAssessmentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>() -> IProvideWinSATAssessmentInfo_Vtbl {
        unsafe extern "system" fn Score<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Score() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(score, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATAssessmentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Score: Score::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATAssessmentInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProvideWinSATResultsInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows::core::Result<IProvideWinSATAssessmentInfo>;
    fn AssessmentState(&self) -> ::windows::core::Result<WINSAT_ASSESSMENT_STATE>;
    fn AssessmentDateTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SystemRating(&self) -> ::windows::core::Result<f32>;
    fn RatingStateDesc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IProvideWinSATResultsInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProvideWinSATResultsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>() -> IProvideWinSATResultsInfo_Vtbl {
        unsafe extern "system" fn GetAssessmentInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAssessmentInfo(::core::mem::transmute_copy(&assessment)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssessmentState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentDateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssessmentDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRating<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SystemRating() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(level, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingStateDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATResultsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RatingStateDesc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAssessmentInfo: GetAssessmentInfo::<Identity, Impl, OFFSET>,
            AssessmentState: AssessmentState::<Identity, Impl, OFFSET>,
            AssessmentDateTime: AssessmentDateTime::<Identity, Impl, OFFSET>,
            SystemRating: SystemRating::<Identity, Impl, OFFSET>,
            RatingStateDesc: RatingStateDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATResultsInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IProvideWinSATVisuals_Impl: Sized {
    fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::RuntimeName for IProvideWinSATVisuals {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IProvideWinSATVisuals_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: isize>() -> IProvideWinSATVisuals_Vtbl {
        unsafe extern "system" fn get_Bitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideWinSATVisuals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Bitmap(::core::mem::transmute_copy(&bitmapsize), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&rating)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitmap, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), get_Bitmap: get_Bitmap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATVisuals as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IQueryAllWinSATAssessments_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_AllXML(&self, xpath: &super::super::Foundation::BSTR, namespaces: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IQueryAllWinSATAssessments {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IQueryAllWinSATAssessments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>() -> IQueryAllWinSATAssessments_Vtbl {
        unsafe extern "system" fn get_AllXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryAllWinSATAssessments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_AllXML(::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomnodelist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), get_AllXML: get_AllXML::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryAllWinSATAssessments as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IQueryOEMWinSATCustomization_Impl: Sized {
    fn GetOEMPrePopulationInfo(&self) -> ::windows::core::Result<WINSAT_OEM_DATA_TYPE>;
}
impl ::windows::core::RuntimeName for IQueryOEMWinSATCustomization {}
impl IQueryOEMWinSATCustomization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>() -> IQueryOEMWinSATCustomization_Vtbl {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryOEMWinSATCustomization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOEMPrePopulationInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetOEMPrePopulationInfo: GetOEMPrePopulationInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryOEMWinSATCustomization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IQueryRecentWinSATAssessment_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_XML(&self, xpath: &super::super::Foundation::BSTR, namespaces: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
    fn Info(&self) -> ::windows::core::Result<IProvideWinSATResultsInfo>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IQueryRecentWinSATAssessment {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IQueryRecentWinSATAssessment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>() -> IQueryRecentWinSATAssessment_Vtbl {
        unsafe extern "system" fn get_XML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_XML(::core::mem::transmute(&xpath), ::core::mem::transmute(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdomnodelist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IQueryRecentWinSATAssessment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Info() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwinsatassessmentinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_XML: get_XML::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryRecentWinSATAssessment as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWinSATInitiateEvents_Impl: Sized {
    fn WinSATComplete(&self, hresult: ::windows::core::HRESULT, strdescription: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn WinSATUpdate(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinSATInitiateEvents {}
impl IWinSATInitiateEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>() -> IWinSATInitiateEvents_Vtbl {
        unsafe extern "system" fn WinSATComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, strdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WinSATComplete(::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&strdescription)).into()
        }
        unsafe extern "system" fn WinSATUpdate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinSATInitiateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WinSATUpdate(::core::mem::transmute_copy(&ucurrenttick), ::core::mem::transmute_copy(&uticktotal), ::core::mem::transmute(&strcurrentstate)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WinSATComplete: WinSATComplete::<Identity, Impl, OFFSET>,
            WinSATUpdate: WinSATUpdate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinSATInitiateEvents as ::windows::core::Interface>::IID
    }
}
