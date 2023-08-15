#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Accessibility\"`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(transparent)]
pub struct IAccessibleWinSAT(::windows_core::IUnknown);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accParent(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accParent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accChildCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accChildCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accChild(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accChild)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accName(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accValue(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accDescription(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accRole(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accRole)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accState(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accHelp(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accHelp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accHelpTopic(&self, pszhelpfile: *mut ::windows_core::BSTR, varchild: super::Variant::VARIANT, pidtopic: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.get_accHelpTopic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszhelpfile), ::core::mem::transmute(varchild), pidtopic).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accKeyboardShortcut(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accKeyboardShortcut)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accFocus(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accFocus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accSelection(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accSelection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn get_accDefaultAction(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_accDefaultAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accSelect(&self, flagsselect: i32, varchild: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accSelect)(::windows_core::Interface::as_raw(self), flagsselect, ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accLocation(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accLocation)(::windows_core::Interface::as_raw(self), pxleft, pytop, pcxwidth, pcyheight, ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accNavigate(&self, navdir: i32, varstart: super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accNavigate)(::windows_core::Interface::as_raw(self), navdir, ::core::mem::transmute(varstart), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.accHitTest)(::windows_core::Interface::as_raw(self), xleft, ytop, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accDoDefaultAction(&self, varchild: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.accDoDefaultAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn put_accName<P0>(&self, varchild: super::Variant::VARIANT, szname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_accName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), szname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn put_accValue<P0>(&self, varchild: super::Variant::VARIANT, szvalue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_accValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varchild), szvalue.into_param().abi()).ok()
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
::windows_core::imp::interface_hierarchy!(IAccessibleWinSAT, ::windows_core::IUnknown, super::Com::IDispatch, super::super::UI::Accessibility::IAccessible);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::cmp::PartialEq for IAccessibleWinSAT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::cmp::Eq for IAccessibleWinSAT {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::fmt::Debug for IAccessibleWinSAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleWinSAT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
unsafe impl ::windows_core::Interface for IAccessibleWinSAT {
    type Vtable = IAccessibleWinSAT_Vtbl;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::clone::Clone for IAccessibleWinSAT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
unsafe impl ::windows_core::ComInterface for IAccessibleWinSAT {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30e6018a_94a8_4ff8_a69a_71b67413f07b);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWinSAT_Vtbl {
    pub base__: super::super::UI::Accessibility::IAccessible_Vtbl,
    pub SetAccessiblityData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsname: ::windows_core::PCWSTR, wsvalue: ::windows_core::PCWSTR, wsdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IInitiateWinSATAssessment(::windows_core::IUnknown);
impl IInitiateWinSATAssessment {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitiateAssessment<P0, P1, P2>(&self, cmdline: P0, pcallbacks: P1, callerhwnd: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWinSATInitiateEvents>,
        P2: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).InitiateAssessment)(::windows_core::Interface::as_raw(self), cmdline.into_param().abi(), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
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
::windows_core::imp::interface_hierarchy!(IInitiateWinSATAssessment, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IInitiateWinSATAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitiateWinSATAssessment {}
impl ::core::fmt::Debug for IInitiateWinSATAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitiateWinSATAssessment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitiateWinSATAssessment {
    type Vtable = IInitiateWinSATAssessment_Vtbl;
}
impl ::core::clone::Clone for IInitiateWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInitiateWinSATAssessment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd983fc50_f5bf_49d5_b5ed_cccb18aa7fc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitiateWinSATAssessment_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmdline: ::windows_core::PCWSTR, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateAssessment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateFormalAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateFormalAssessment: usize,
    pub CancelAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProvideWinSATAssessmentInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfo {
    pub unsafe fn Score(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Score)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProvideWinSATAssessmentInfo, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IProvideWinSATAssessmentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IProvideWinSATAssessmentInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IProvideWinSATAssessmentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATAssessmentInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IProvideWinSATAssessmentInfo {
    type Vtable = IProvideWinSATAssessmentInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProvideWinSATAssessmentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IProvideWinSATAssessmentInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cd1c380_52d3_4678_ac6f_e929e480be9e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATAssessmentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Score: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProvideWinSATResultsInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows_core::Result<IProvideWinSATAssessmentInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAssessmentInfo)(::windows_core::Interface::as_raw(self), assessment, &mut result__).from_abi(result__)
    }
    pub unsafe fn AssessmentState(&self) -> ::windows_core::Result<WINSAT_ASSESSMENT_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AssessmentDateTime(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssessmentDateTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SystemRating(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SystemRating)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RatingStateDesc(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RatingStateDesc)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IProvideWinSATResultsInfo, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IProvideWinSATResultsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IProvideWinSATResultsInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IProvideWinSATResultsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATResultsInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IProvideWinSATResultsInfo {
    type Vtable = IProvideWinSATResultsInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProvideWinSATResultsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IProvideWinSATResultsInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8334d5d_568e_4075_875f_9df341506640);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATResultsInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAssessmentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAssessmentInfo: usize,
    pub AssessmentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AssessmentDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AssessmentDateTime: usize,
    pub SystemRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows_core::HRESULT,
    pub RatingStateDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IProvideWinSATVisuals(::windows_core::IUnknown);
impl IProvideWinSATVisuals {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn get_Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Bitmap)(::windows_core::Interface::as_raw(self), bitmapsize, state, rating, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IProvideWinSATVisuals, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IProvideWinSATVisuals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideWinSATVisuals {}
impl ::core::fmt::Debug for IProvideWinSATVisuals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideWinSATVisuals").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProvideWinSATVisuals {
    type Vtable = IProvideWinSATVisuals_Vtbl;
}
impl ::core::clone::Clone for IProvideWinSATVisuals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IProvideWinSATVisuals {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9f4ade0_871a_42a3_b813_3078d25162c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATVisuals_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub get_Bitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    get_Bitmap: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IQueryAllWinSATAssessments(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IQueryAllWinSATAssessments {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn get_AllXML<P0, P1>(&self, xpath: P0, namespaces: P1) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_AllXML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IQueryAllWinSATAssessments, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IQueryAllWinSATAssessments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IQueryAllWinSATAssessments {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IQueryAllWinSATAssessments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryAllWinSATAssessments").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IQueryAllWinSATAssessments {
    type Vtable = IQueryAllWinSATAssessments_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IQueryAllWinSATAssessments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IQueryAllWinSATAssessments {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b89ed1d_6398_4fea_87fc_567d8d19176f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAllWinSATAssessments_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub get_AllXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaces: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    get_AllXML: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IQueryOEMWinSATCustomization(::windows_core::IUnknown);
impl IQueryOEMWinSATCustomization {
    pub unsafe fn GetOEMPrePopulationInfo(&self) -> ::windows_core::Result<WINSAT_OEM_CUSTOMIZATION_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOEMPrePopulationInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IQueryOEMWinSATCustomization, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IQueryOEMWinSATCustomization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryOEMWinSATCustomization {}
impl ::core::fmt::Debug for IQueryOEMWinSATCustomization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryOEMWinSATCustomization").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IQueryOEMWinSATCustomization {
    type Vtable = IQueryOEMWinSATCustomization_Vtbl;
}
impl ::core::clone::Clone for IQueryOEMWinSATCustomization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQueryOEMWinSATCustomization {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc9a6a9f_ad4e_420e_9953_b34671e9df22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOEMWinSATCustomization_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_CUSTOMIZATION_STATE) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IQueryRecentWinSATAssessment(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IQueryRecentWinSATAssessment {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn get_XML<P0, P1>(&self, xpath: P0, namespaces: P1) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_XML)(::windows_core::Interface::as_raw(self), xpath.into_param().abi(), namespaces.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Info(&self) -> ::windows_core::Result<IProvideWinSATResultsInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Info)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IQueryRecentWinSATAssessment, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IQueryRecentWinSATAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IQueryRecentWinSATAssessment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IQueryRecentWinSATAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryRecentWinSATAssessment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IQueryRecentWinSATAssessment {
    type Vtable = IQueryRecentWinSATAssessment_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IQueryRecentWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IQueryRecentWinSATAssessment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8ad5d1f_3b47_4bdc_9375_7c6b1da4eca7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryRecentWinSATAssessment_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub get_XML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaces: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    get_XML: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Info: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IWinSATInitiateEvents(::windows_core::IUnknown);
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
::windows_core::imp::interface_hierarchy!(IWinSATInitiateEvents, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IWinSATInitiateEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinSATInitiateEvents {}
impl ::core::fmt::Debug for IWinSATInitiateEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinSATInitiateEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWinSATInitiateEvents {
    type Vtable = IWinSATInitiateEvents_Vtbl;
}
impl ::core::clone::Clone for IWinSATInitiateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWinSATInitiateEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x262a1918_ba0d_41d5_92c2_fab4633ee74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinSATInitiateEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub WinSATComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, strdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CAccessiblityWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CInitiateWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CProvideWinSATVisuals: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CQueryAllWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CQueryOEMWinSATCustomization: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const CQueryWinSAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_CUSTOMIZATION_STATE = WINSAT_OEM_CUSTOMIZATION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
impl ::core::marker::Copy for WINSAT_ASSESSMENT_STATE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_ASSESSMENT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINSAT_ASSESSMENT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
impl ::core::marker::Copy for WINSAT_ASSESSMENT_TYPE {}
impl ::core::clone::Clone for WINSAT_ASSESSMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_ASSESSMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINSAT_ASSESSMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
impl ::core::marker::Copy for WINSAT_BITMAP_SIZE {}
impl ::core::clone::Clone for WINSAT_BITMAP_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_BITMAP_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WINSAT_BITMAP_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WINSAT_BITMAP_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_BITMAP_SIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINSAT_OEM_CUSTOMIZATION_STATE(pub i32);
impl ::core::marker::Copy for WINSAT_OEM_CUSTOMIZATION_STATE {}
impl ::core::clone::Clone for WINSAT_OEM_CUSTOMIZATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_OEM_CUSTOMIZATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
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
