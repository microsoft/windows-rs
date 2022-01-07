#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSATImpl: Sized + IAccessibleImpl + IDispatchImpl {
    fn SetAccessiblityData();
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::windows::core::RuntimeName for IAccessibleWinSAT {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IAccessibleWinSAT";
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSATVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWinSATImpl, const OFFSET: isize>() -> IAccessibleWinSATVtbl {
        unsafe extern "system" fn SetAccessiblityData<Impl: IAccessibleWinSATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsname: super::super::Foundation::PWSTR, wsvalue: super::super::Foundation::PWSTR, wsdesc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAccessiblityData(
                &*(&wsname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wsvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wsdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessibleWinSAT>, ::windows::core::GetTrustLevel, SetAccessiblityData::<Impl, OFFSET>)
    }
}
pub trait IInitiateWinSATAssessmentImpl: Sized {
    fn InitiateAssessment();
    fn InitiateFormalAssessment();
    fn CancelAssessment();
}
impl ::windows::core::RuntimeName for IInitiateWinSATAssessment {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IInitiateWinSATAssessment";
}
impl IInitiateWinSATAssessmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>() -> IInitiateWinSATAssessmentVtbl {
        unsafe extern "system" fn InitiateAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmdline: super::super::Foundation::PWSTR, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitiateAssessment(
                &*(&cmdline as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallbacks as *const <IWinSATInitiateEvents as ::windows::core::Abi>::Abi as *const <IWinSATInitiateEvents as ::windows::core::DefaultType>::DefaultType),
                &*(&callerhwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitiateFormalAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitiateFormalAssessment(&*(&pcallbacks as *const <IWinSATInitiateEvents as ::windows::core::Abi>::Abi as *const <IWinSATInitiateEvents as ::windows::core::DefaultType>::DefaultType), &*(&callerhwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAssessment<Impl: IInitiateWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAssessment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInitiateWinSATAssessment>, ::windows::core::GetTrustLevel, InitiateAssessment::<Impl, OFFSET>, InitiateFormalAssessment::<Impl, OFFSET>, CancelAssessment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATAssessmentInfoImpl: Sized + IDispatchImpl {
    fn Score();
    fn Title();
    fn Description();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IProvideWinSATAssessmentInfo {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IProvideWinSATAssessmentInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>() -> IProvideWinSATAssessmentInfoVtbl {
        unsafe extern "system" fn Score<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Score(::core::mem::transmute_copy(&score)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IProvideWinSATAssessmentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideWinSATAssessmentInfo>, ::windows::core::GetTrustLevel, Score::<Impl, OFFSET>, Title::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATResultsInfoImpl: Sized + IDispatchImpl {
    fn GetAssessmentInfo();
    fn AssessmentState();
    fn AssessmentDateTime();
    fn SystemRating();
    fn RatingStateDesc();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IProvideWinSATResultsInfo {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IProvideWinSATResultsInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>() -> IProvideWinSATResultsInfoVtbl {
        unsafe extern "system" fn GetAssessmentInfo<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssessmentInfo(assessment, ::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentState<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssessmentState(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssessmentDateTime<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssessmentDateTime(::core::mem::transmute_copy(&filetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRating<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRating(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingStateDesc<Impl: IProvideWinSATResultsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RatingStateDesc(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideWinSATResultsInfo>, ::windows::core::GetTrustLevel, GetAssessmentInfo::<Impl, OFFSET>, AssessmentState::<Impl, OFFSET>, AssessmentDateTime::<Impl, OFFSET>, SystemRating::<Impl, OFFSET>, RatingStateDesc::<Impl, OFFSET>)
    }
}
pub trait IProvideWinSATVisualsImpl: Sized {
    fn Bitmap();
}
impl ::windows::core::RuntimeName for IProvideWinSATVisuals {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IProvideWinSATVisuals";
}
impl IProvideWinSATVisualsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideWinSATVisualsImpl, const OFFSET: isize>() -> IProvideWinSATVisualsVtbl {
        unsafe extern "system" fn Bitmap<Impl: IProvideWinSATVisualsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bitmap(bitmapsize, state, rating, ::core::mem::transmute_copy(&pbitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvideWinSATVisuals>, ::windows::core::GetTrustLevel, Bitmap::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IQueryAllWinSATAssessmentsImpl: Sized + IDispatchImpl {
    fn AllXML();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IQueryAllWinSATAssessments {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IQueryAllWinSATAssessments";
}
#[cfg(feature = "Win32_System_Com")]
impl IQueryAllWinSATAssessmentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryAllWinSATAssessmentsImpl, const OFFSET: isize>() -> IQueryAllWinSATAssessmentsVtbl {
        unsafe extern "system" fn AllXML<Impl: IQueryAllWinSATAssessmentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllXML(&*(&xpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaces as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdomnodelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQueryAllWinSATAssessments>, ::windows::core::GetTrustLevel, AllXML::<Impl, OFFSET>)
    }
}
pub trait IQueryOEMWinSATCustomizationImpl: Sized {
    fn GetOEMPrePopulationInfo();
}
impl ::windows::core::RuntimeName for IQueryOEMWinSATCustomization {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IQueryOEMWinSATCustomization";
}
impl IQueryOEMWinSATCustomizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryOEMWinSATCustomizationImpl, const OFFSET: isize>() -> IQueryOEMWinSATCustomizationVtbl {
        unsafe extern "system" fn GetOEMPrePopulationInfo<Impl: IQueryOEMWinSATCustomizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOEMPrePopulationInfo(::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQueryOEMWinSATCustomization>, ::windows::core::GetTrustLevel, GetOEMPrePopulationInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IQueryRecentWinSATAssessmentImpl: Sized + IDispatchImpl {
    fn XML();
    fn Info();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IQueryRecentWinSATAssessment {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IQueryRecentWinSATAssessment";
}
#[cfg(feature = "Win32_System_Com")]
impl IQueryRecentWinSATAssessmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryRecentWinSATAssessmentImpl, const OFFSET: isize>() -> IQueryRecentWinSATAssessmentVtbl {
        unsafe extern "system" fn XML<Impl: IQueryRecentWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XML(&*(&xpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaces as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdomnodelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IQueryRecentWinSATAssessmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info(::core::mem::transmute_copy(&ppwinsatassessmentinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IQueryRecentWinSATAssessment>, ::windows::core::GetTrustLevel, XML::<Impl, OFFSET>, Info::<Impl, OFFSET>)
    }
}
pub trait IWinSATInitiateEventsImpl: Sized {
    fn WinSATComplete();
    fn WinSATUpdate();
}
impl ::windows::core::RuntimeName for IWinSATInitiateEvents {
    const NAME: &'static str = "Windows.Win32.System.AssessmentTool.IWinSATInitiateEvents";
}
impl IWinSATInitiateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinSATInitiateEventsImpl, const OFFSET: isize>() -> IWinSATInitiateEventsVtbl {
        unsafe extern "system" fn WinSATComplete<Impl: IWinSATInitiateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, strdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WinSATComplete(hresult, &*(&strdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WinSATUpdate<Impl: IWinSATInitiateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WinSATUpdate(ucurrenttick, uticktotal, &*(&strcurrentstate as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinSATInitiateEvents>, ::windows::core::GetTrustLevel, WinSATComplete::<Impl, OFFSET>, WinSATUpdate::<Impl, OFFSET>)
    }
}
