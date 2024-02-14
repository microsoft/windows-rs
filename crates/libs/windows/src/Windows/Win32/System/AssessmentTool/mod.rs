#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
::windows_core::imp::com_interface!(IAccessibleWinSAT, IAccessibleWinSAT_Vtbl, 0x30e6018a_94a8_4ff8_a69a_71b67413f07b);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
::windows_core::imp::interface_hierarchy!(IAccessibleWinSAT, ::windows_core::IUnknown, super::Com::IDispatch, super::super::UI::Accessibility::IAccessible);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accParent(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accParent)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accChildCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accChildCount)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accChild<P0>(&self, varchild: P0) -> ::windows_core::Result<super::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accChild)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accName<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accName)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accValue<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accValue)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accDescription<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDescription)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accRole<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accRole)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accState<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accState)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accHelp<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accHelp)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accHelpTopic<P0>(&self, pszhelpfile: *mut ::windows_core::BSTR, varchild: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accHelpTopic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszhelpfile), varchild.into_param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accKeyboardShortcut<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accKeyboardShortcut)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accFocus(&self) -> ::windows_core::Result<::windows_core::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accFocus)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accSelection(&self) -> ::windows_core::Result<::windows_core::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accSelection)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn get_accDefaultAction<P0>(&self, varchild: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDefaultAction)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accSelect<P0>(&self, flagsselect: i32, varchild: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        (::windows_core::Interface::vtable(self).base__.accSelect)(::windows_core::Interface::as_raw(self), flagsselect, varchild.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accLocation<P0>(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        (::windows_core::Interface::vtable(self).base__.accLocation)(::windows_core::Interface::as_raw(self), pxleft, pytop, pcxwidth, pcyheight, varchild.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accNavigate<P0>(&self, navdir: i32, varstart: P0) -> ::windows_core::Result<::windows_core::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accNavigate)(::windows_core::Interface::as_raw(self), navdir, varstart.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows_core::Result<::windows_core::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accHitTest)(::windows_core::Interface::as_raw(self), xleft, ytop, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accDoDefaultAction<P0>(&self, varchild: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        (::windows_core::Interface::vtable(self).base__.accDoDefaultAction)(::windows_core::Interface::as_raw(self), varchild.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn put_accName<P0, P1>(&self, varchild: P0, szname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_accName)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), szname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn put_accValue<P0, P1>(&self, varchild: P0, szvalue: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::VARIANT>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_accValue)(::windows_core::Interface::as_raw(self), varchild.into_param().abi(), szvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetAccessiblityData<P0, P1, P2>(&self, wsname: P0, wsvalue: P1, wsdesc: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccessiblityData)(::windows_core::Interface::as_raw(self), wsname.into_param().abi(), wsvalue.into_param().abi(), wsdesc.into_param().abi()).ok()
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(C)]
pub struct IAccessibleWinSAT_Vtbl {
    pub base__: super::super::UI::Accessibility::IAccessible_Vtbl,
    pub SetAccessiblityData: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IInitiateWinSATAssessment, IInitiateWinSATAssessment_Vtbl, 0xd983fc50_f5bf_49d5_b5ed_cccb18aa7fc1);
::windows_core::imp::interface_hierarchy!(IInitiateWinSATAssessment, ::windows_core::IUnknown);
impl IInitiateWinSATAssessment {
    pub unsafe fn InitiateAssessment<P0, P1, P2>(&self, cmdline: P0, pcallbacks: P1, callerhwnd: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWinSATInitiateEvents>,
        P2: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).InitiateAssessment)(::windows_core::Interface::as_raw(self), cmdline.into_param().abi(), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    pub unsafe fn InitiateFormalAssessment<P0, P1>(&self, pcallbacks: P0, callerhwnd: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWinSATInitiateEvents>,
        P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).InitiateFormalAssessment)(::windows_core::Interface::as_raw(self), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    pub unsafe fn CancelAssessment(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAssessment)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInitiateWinSATAssessment_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub InitiateAssessment: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *mut ::core::ffi::c_void, super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    pub InitiateFormalAssessment: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    pub CancelAssessment: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(IProvideWinSATAssessmentInfo, IProvideWinSATAssessmentInfo_Vtbl, 0x0cd1c380_52d3_4678_ac6f_e929e480be9e);
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProvideWinSATAssessmentInfo, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfo {
    pub unsafe fn Score(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Score)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProvideWinSATAssessmentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Score: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut f32) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(IProvideWinSATResultsInfo, IProvideWinSATResultsInfo_Vtbl, 0xf8334d5d_568e_4075_875f_9df341506640);
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProvideWinSATResultsInfo, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows_core::Result<IProvideWinSATAssessmentInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAssessmentInfo)(::windows_core::Interface::as_raw(self), assessment, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AssessmentState(&self) -> ::windows_core::Result<WINSAT_ASSESSMENT_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentState)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AssessmentDateTime(&self) -> ::windows_core::Result<::windows_core::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentDateTime)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SystemRating(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SystemRating)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RatingStateDesc(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RatingStateDesc)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProvideWinSATResultsInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAssessmentInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, WINSAT_ASSESSMENT_TYPE, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAssessmentInfo: usize,
    pub AssessmentState: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut WINSAT_ASSESSMENT_STATE) -> ::windows_core::HRESULT,
    pub AssessmentDateTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub SystemRating: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut f32) -> ::windows_core::HRESULT,
    pub RatingStateDesc: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IProvideWinSATVisuals, IProvideWinSATVisuals_Vtbl, 0xa9f4ade0_871a_42a3_b813_3078d25162c9);
::windows_core::imp::interface_hierarchy!(IProvideWinSATVisuals, ::windows_core::IUnknown);
impl IProvideWinSATVisuals {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Bitmap)(::windows_core::Interface::as_raw(self), bitmapsize, state, rating, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProvideWinSATVisuals_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub get_Bitmap: unsafe extern "system" fn(*mut ::core::ffi::c_void, WINSAT_BITMAP_SIZE, WINSAT_ASSESSMENT_STATE, f32, *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    get_Bitmap: usize,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(IQueryAllWinSATAssessments, IQueryAllWinSATAssessments_Vtbl, 0x0b89ed1d_6398_4fea_87fc_567d8d19176f);
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IQueryAllWinSATAssessments, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IQueryAllWinSATAssessments {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn get_AllXML<P0, P1>(&self, xpath: P0, namespaces: P1) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_AllXML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IQueryAllWinSATAssessments_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub get_AllXML: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    get_AllXML: usize,
}
::windows_core::imp::com_interface!(IQueryOEMWinSATCustomization, IQueryOEMWinSATCustomization_Vtbl, 0xbc9a6a9f_ad4e_420e_9953_b34671e9df22);
::windows_core::imp::interface_hierarchy!(IQueryOEMWinSATCustomization, ::windows_core::IUnknown);
impl IQueryOEMWinSATCustomization {
    pub unsafe fn GetOEMPrePopulationInfo(&self) -> ::windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOEMPrePopulationInfo)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IQueryOEMWinSATCustomization_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::com_interface!(IQueryRecentWinSATAssessment, IQueryRecentWinSATAssessment_Vtbl, 0xf8ad5d1f_3b47_4bdc_9375_7c6b1da4eca7);
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IQueryRecentWinSATAssessment, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IQueryRecentWinSATAssessment {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn get_XML<P0, P1>(&self, xpath: P0, namespaces: P1) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_XML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Info(&self) -> ::windows_core::Result<IProvideWinSATResultsInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Info)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IQueryRecentWinSATAssessment_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub get_XML: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    get_XML: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Info: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Info: usize,
}
::windows_core::imp::com_interface!(IWinSATInitiateEvents, IWinSATInitiateEvents_Vtbl, 0x262a1918_ba0d_41d5_92c2_fab4633ee74f);
::windows_core::imp::interface_hierarchy!(IWinSATInitiateEvents, ::windows_core::IUnknown);
impl IWinSATInitiateEvents {
    pub unsafe fn WinSATComplete<P0>(&self, hresult: ::windows_core::HRESULT, strdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WinSATComplete)(::windows_core::Interface::as_raw(self), hresult, strdescription.into_param().abi()).ok()
    }
    pub unsafe fn WinSATUpdate<P0>(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WinSATUpdate)(::windows_core::Interface::as_raw(self), ucurrenttick, uticktotal, strcurrentstate.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWinSATInitiateEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub WinSATComplete: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::HRESULT, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, u32, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
pub const CAccessiblityWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
pub const CInitiateWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
pub const CProvideWinSATVisuals: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
pub const CQueryAllWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
pub const CQueryOEMWinSATCustomization: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
pub const CQueryWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(2i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(1i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(0i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(3i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
impl ::windows_core::TypeKind for WINSAT_ASSESSMENT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
impl ::windows_core::TypeKind for WINSAT_ASSESSMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
impl ::windows_core::TypeKind for WINSAT_BITMAP_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_BITMAP_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_BITMAP_SIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WINSAT_OEM_CUSTOMIZATION_STATE(pub i32);
impl ::windows_core::TypeKind for WINSAT_OEM_CUSTOMIZATION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_OEM_CUSTOMIZATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_OEM_CUSTOMIZATION_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
