#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSATImpl: Sized + IDispatchImpl + IAccessibleImpl {
    fn SetAccessiblityData(&mut self, wsname: super::super::Foundation::PWSTR, wsvalue: super::super::Foundation::PWSTR, wsdesc: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSATVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWinSATImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleWinSATVtbl {
        unsafe extern "system" fn SetAccessiblityData<Impl: IAccessibleWinSATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsname: super::super::Foundation::PWSTR, wsvalue: super::super::Foundation::PWSTR, wsdesc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessiblityData(::core::mem::transmute_copy(&wsname), ::core::mem::transmute_copy(&wsvalue), ::core::mem::transmute_copy(&wsdesc)).into()
        }
        Self { base: IAccessibleVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetAccessiblityData: SetAccessiblityData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleWinSAT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInitiateWinSATAssessmentImpl: Sized {
    fn InitiateAssessment(&mut self, cmdline: super::super::Foundation::PWSTR, pcallbacks: ::core::option::Option<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn InitiateFormalAssessment(&mut self, pcallbacks: ::core::option::Option<IWinSATInitiateEvents>, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn CancelAssessment(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInitiateWinSATAssessmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitiateWinSATAssessmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitiateWinSATAssessmentVtbl {
        unsafe extern "system" fn InitiateAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmdline: super::super::Foundation::PWSTR, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitiateAssessment(::core::mem::transmute_copy(&cmdline), ::core::mem::transmute(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn InitiateFormalAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitiateFormalAssessment(::core::mem::transmute(&pcallbacks), ::core::mem::transmute_copy(&callerhwnd)).into()
        }
        unsafe extern "system" fn CancelAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAssessment().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitiateAssessment: InitiateAssessment::<Impl, IMPL_OFFSET>,
            InitiateFormalAssessment: InitiateFormalAssessment::<Impl, IMPL_OFFSET>,
            CancelAssessment: CancelAssessment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitiateWinSATAssessment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProvideWinSATAssessmentInfoImpl: Sized + IDispatchImpl {
    fn Score(&mut self) -> ::windows::core::Result<f32>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProvideWinSATAssessmentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATAssessmentInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideWinSATAssessmentInfoVtbl {
        unsafe extern "system" fn Score<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Score() {
                ::core::result::Result::Ok(ok__) => {
                    *score = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Score: Score::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATAssessmentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProvideWinSATResultsInfoImpl: Sized + IDispatchImpl {
    fn GetAssessmentInfo(&mut self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows::core::Result<IProvideWinSATAssessmentInfo>;
    fn AssessmentState(&mut self) -> ::windows::core::Result<WINSAT_ASSESSMENT_STATE>;
    fn AssessmentDateTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SystemRating(&mut self) -> ::windows::core::Result<f32>;
    fn RatingStateDesc(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProvideWinSATResultsInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATResultsInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideWinSATResultsInfoVtbl {
        unsafe extern "system" fn GetAssessmentInfo<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssessmentInfo(::core::mem::transmute_copy(&assessment)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentState<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssessmentState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentDateTime<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssessmentDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *filetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRating<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRating() {
                ::core::result::Result::Ok(ok__) => {
                    *level = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingStateDesc<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RatingStateDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAssessmentInfo: GetAssessmentInfo::<Impl, IMPL_OFFSET>,
            AssessmentState: AssessmentState::<Impl, IMPL_OFFSET>,
            AssessmentDateTime: AssessmentDateTime::<Impl, IMPL_OFFSET>,
            SystemRating: SystemRating::<Impl, IMPL_OFFSET>,
            RatingStateDesc: RatingStateDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATResultsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IProvideWinSATVisualsImpl: Sized {
    fn Bitmap(&mut self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IProvideWinSATVisualsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATVisualsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideWinSATVisualsVtbl {
        unsafe extern "system" fn Bitmap<Impl: IProvideWinSATVisualsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bitmap(::core::mem::transmute_copy(&bitmapsize), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&rating)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Bitmap: Bitmap::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideWinSATVisuals as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IQueryAllWinSATAssessmentsImpl: Sized + IDispatchImpl {
    fn AllXML(&mut self, xpath: super::super::Foundation::BSTR, namespaces: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IQueryAllWinSATAssessmentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryAllWinSATAssessmentsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryAllWinSATAssessmentsVtbl {
        unsafe extern "system" fn AllXML<Impl: IQueryAllWinSATAssessmentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllXML(::core::mem::transmute_copy(&xpath), ::core::mem::transmute_copy(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdomnodelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), AllXML: AllXML::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryAllWinSATAssessments as ::windows::core::Interface>::IID
    }
}
pub trait IQueryOEMWinSATCustomizationImpl: Sized {
    fn GetOEMPrePopulationInfo(&mut self) -> ::windows::core::Result<WINSAT_OEM_DATA_TYPE>;
}
impl IQueryOEMWinSATCustomizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOEMWinSATCustomizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryOEMWinSATCustomizationVtbl {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Impl: IQueryOEMWinSATCustomizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOEMPrePopulationInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOEMPrePopulationInfo: GetOEMPrePopulationInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryOEMWinSATCustomization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IQueryRecentWinSATAssessmentImpl: Sized + IDispatchImpl {
    fn XML(&mut self, xpath: super::super::Foundation::BSTR, namespaces: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
    fn Info(&mut self) -> ::windows::core::Result<IProvideWinSATResultsInfo>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IQueryRecentWinSATAssessmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryRecentWinSATAssessmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryRecentWinSATAssessmentVtbl {
        unsafe extern "system" fn XML<Impl: IQueryRecentWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XML(::core::mem::transmute_copy(&xpath), ::core::mem::transmute_copy(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdomnodelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IQueryRecentWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwinsatassessmentinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), XML: XML::<Impl, IMPL_OFFSET>, Info: Info::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryRecentWinSATAssessment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWinSATInitiateEventsImpl: Sized {
    fn WinSATComplete(&mut self, hresult: ::windows::core::HRESULT, strdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn WinSATUpdate(&mut self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinSATInitiateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinSATInitiateEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinSATInitiateEventsVtbl {
        unsafe extern "system" fn WinSATComplete<Impl: IWinSATInitiateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, strdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WinSATComplete(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&strdescription)).into()
        }
        unsafe extern "system" fn WinSATUpdate<Impl: IWinSATInitiateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WinSATUpdate(::core::mem::transmute_copy(&ucurrenttick), ::core::mem::transmute_copy(&uticktotal), ::core::mem::transmute_copy(&strcurrentstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            WinSATComplete: WinSATComplete::<Impl, IMPL_OFFSET>,
            WinSATUpdate: WinSATUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinSATInitiateEvents as ::windows::core::Interface>::IID
    }
}
