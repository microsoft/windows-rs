#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CAccessiblityWinSAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e18f9c6_a3eb_495a_89b7_956482e19f7a);
pub const CInitiateWinSAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x489331dc_f5e0_4528_9fda_45331bf4a571);
pub const CProvideWinSATVisuals: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f377d7e_e551_44f8_9f94_9db392b03b7b);
pub const CQueryAllWinSAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05df8d13_c355_47f4_a11e_851b338cefb8);
pub const CQueryOEMWinSATCustomization: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc47a41b7_b729_424f_9af9_5cb3934f2dfa);
pub const CQueryWinSAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3bdfad3_f276_49e9_9b17_c474f48f0764);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Accessibility\"`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(transparent)]
pub struct IAccessibleWinSAT(::windows::core::IUnknown);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl IAccessibleWinSAT {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accParent(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accParent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(feature = "Win32_UI_Accessibility")]
    pub unsafe fn accChildCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accChildCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accChild<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accChild)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accName<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accName)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accValue)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accDescription<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accDescription)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accRole<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accRole)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accState<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accState)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accHelp<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accHelp)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accHelpTopic<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: Param1, pidtopic: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.accHelpTopic)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszhelpfile), varchild.into_param().abi(), ::core::mem::transmute(pidtopic)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accKeyboardShortcut<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accKeyboardShortcut)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accFocus(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accFocus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accSelection(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accDefaultAction<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accDefaultAction)(::core::mem::transmute_copy(self), varchild.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accSelect<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, flagsselect: i32, varchild: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.accSelect)(::core::mem::transmute_copy(self), ::core::mem::transmute(flagsselect), varchild.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accLocation<'a, Param4: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.accLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(pxleft), ::core::mem::transmute(pytop), ::core::mem::transmute(pcxwidth), ::core::mem::transmute(pcyheight), varchild.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accNavigate<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, navdir: i32, varstart: Param1) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accNavigate)(::core::mem::transmute_copy(self), ::core::mem::transmute(navdir), varstart.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.accHitTest)(::core::mem::transmute_copy(self), ::core::mem::transmute(xleft), ::core::mem::transmute(ytop), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn accDoDefaultAction<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varchild: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.accDoDefaultAction)(::core::mem::transmute_copy(self), varchild.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn SetaccName<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, varchild: Param0, szname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetaccName)(::core::mem::transmute_copy(self), varchild.into_param().abi(), szname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Accessibility\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Accessibility"))]
    pub unsafe fn SetaccValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, varchild: Param0, szvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetaccValue)(::core::mem::transmute_copy(self), varchild.into_param().abi(), szvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn SetAccessiblityData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wsname: Param0, wsvalue: Param1, wsdesc: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccessiblityData)(::core::mem::transmute_copy(self), wsname.into_param().abi(), wsvalue.into_param().abi(), wsdesc.into_param().abi()).ok()
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<IAccessibleWinSAT> for ::windows::core::IUnknown {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<&IAccessibleWinSAT> for ::windows::core::IUnknown {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<IAccessibleWinSAT> for super::Com::IDispatch {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<&IAccessibleWinSAT> for super::Com::IDispatch {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<IAccessibleWinSAT> for super::super::UI::Accessibility::IAccessible {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::convert::From<&IAccessibleWinSAT> for super::super::UI::Accessibility::IAccessible {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::UI::Accessibility::IAccessible> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UI::Accessibility::IAccessible> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::UI::Accessibility::IAccessible> for &'a IAccessibleWinSAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UI::Accessibility::IAccessible> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
impl ::core::clone::Clone for IAccessibleWinSAT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IAccessibleWinSAT {
    type Vtable = IAccessibleWinSAT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30e6018a_94a8_4ff8_a69a_71b67413f07b);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWinSAT_Vtbl {
    pub base: super::super::UI::Accessibility::IAccessible_Vtbl,
    pub SetAccessiblityData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsname: ::windows::core::PCWSTR, wsvalue: ::windows::core::PCWSTR, wsdesc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IInitiateWinSATAssessment(::windows::core::IUnknown);
impl IInitiateWinSATAssessment {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitiateAssessment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWinSATInitiateEvents>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, cmdline: Param0, pcallbacks: Param1, callerhwnd: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitiateAssessment)(::core::mem::transmute_copy(self), cmdline.into_param().abi(), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitiateFormalAssessment<'a, Param0: ::windows::core::IntoParam<'a, IWinSATInitiateEvents>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pcallbacks: Param0, callerhwnd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitiateFormalAssessment)(::core::mem::transmute_copy(self), pcallbacks.into_param().abi(), callerhwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn CancelAssessment(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelAssessment)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IInitiateWinSATAssessment> for ::windows::core::IUnknown {
    fn from(value: IInitiateWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitiateWinSATAssessment> for ::windows::core::IUnknown {
    fn from(value: &IInitiateWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitiateWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitiateWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitiateWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IInitiateWinSATAssessment {
    type Vtable = IInitiateWinSATAssessment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd983fc50_f5bf_49d5_b5ed_cccb18aa7fc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitiateWinSATAssessment_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmdline: ::windows::core::PCWSTR, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateAssessment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateFormalAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr, callerhwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateFormalAssessment: usize,
    pub CancelAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProvideWinSATAssessmentInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATAssessmentInfo {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn Score(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Score)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Title)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProvideWinSATAssessmentInfo> for ::windows::core::IUnknown {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProvideWinSATAssessmentInfo> for ::windows::core::IUnknown {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProvideWinSATAssessmentInfo> for super::Com::IDispatch {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProvideWinSATAssessmentInfo> for super::Com::IDispatch {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IProvideWinSATAssessmentInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProvideWinSATAssessmentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IProvideWinSATAssessmentInfo {
    type Vtable = IProvideWinSATAssessmentInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cd1c380_52d3_4678_ac6f_e929e480be9e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATAssessmentInfo_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    pub Score: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, score: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProvideWinSATResultsInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProvideWinSATResultsInfo {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAssessmentInfo(&self, assessment: WINSAT_ASSESSMENT_TYPE) -> ::windows::core::Result<IProvideWinSATAssessmentInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAssessmentInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(assessment), ::core::mem::transmute(&mut result__)).from_abi::<IProvideWinSATAssessmentInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn AssessmentState(&self) -> ::windows::core::Result<WINSAT_ASSESSMENT_STATE> {
        let mut result__: WINSAT_ASSESSMENT_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AssessmentState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WINSAT_ASSESSMENT_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AssessmentDateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AssessmentDateTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn SystemRating(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SystemRating)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RatingStateDesc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RatingStateDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProvideWinSATResultsInfo> for ::windows::core::IUnknown {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProvideWinSATResultsInfo> for ::windows::core::IUnknown {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProvideWinSATResultsInfo> for super::Com::IDispatch {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProvideWinSATResultsInfo> for super::Com::IDispatch {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IProvideWinSATResultsInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProvideWinSATResultsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IProvideWinSATResultsInfo {
    type Vtable = IProvideWinSATResultsInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8334d5d_568e_4075_875f_9df341506640);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATResultsInfo_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAssessmentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAssessmentInfo: usize,
    pub AssessmentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AssessmentDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AssessmentDateTime: usize,
    pub SystemRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RatingStateDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RatingStateDesc: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IProvideWinSATVisuals(::windows::core::IUnknown);
impl IProvideWinSATVisuals {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Bitmap(&self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: super::super::Graphics::Gdi::HBITMAP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Bitmap)(::core::mem::transmute_copy(self), ::core::mem::transmute(bitmapsize), ::core::mem::transmute(state), ::core::mem::transmute(rating), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
impl ::core::convert::From<IProvideWinSATVisuals> for ::windows::core::IUnknown {
    fn from(value: IProvideWinSATVisuals) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideWinSATVisuals> for ::windows::core::IUnknown {
    fn from(value: &IProvideWinSATVisuals) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvideWinSATVisuals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProvideWinSATVisuals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvideWinSATVisuals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IProvideWinSATVisuals {
    type Vtable = IProvideWinSATVisuals_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9f4ade0_871a_42a3_b813_3078d25162c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATVisuals_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Bitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Bitmap: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IQueryAllWinSATAssessments(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IQueryAllWinSATAssessments {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AllXML<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllXML)(::core::mem::transmute_copy(self), xpath.into_param().abi(), namespaces.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IQueryAllWinSATAssessments> for ::windows::core::IUnknown {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IQueryAllWinSATAssessments> for ::windows::core::IUnknown {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IQueryAllWinSATAssessments> for super::Com::IDispatch {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IQueryAllWinSATAssessments> for super::Com::IDispatch {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IQueryAllWinSATAssessments {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IQueryAllWinSATAssessments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IQueryAllWinSATAssessments {
    type Vtable = IQueryAllWinSATAssessments_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b89ed1d_6398_4fea_87fc_567d8d19176f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAllWinSATAssessments_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AllXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AllXML: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IQueryOEMWinSATCustomization(::windows::core::IUnknown);
impl IQueryOEMWinSATCustomization {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn GetOEMPrePopulationInfo(&self) -> ::windows::core::Result<WINSAT_OEM_DATA_TYPE> {
        let mut result__: WINSAT_OEM_DATA_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOEMPrePopulationInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WINSAT_OEM_DATA_TYPE>(result__)
    }
}
impl ::core::convert::From<IQueryOEMWinSATCustomization> for ::windows::core::IUnknown {
    fn from(value: IQueryOEMWinSATCustomization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IQueryOEMWinSATCustomization> for ::windows::core::IUnknown {
    fn from(value: &IQueryOEMWinSATCustomization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IQueryOEMWinSATCustomization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IQueryOEMWinSATCustomization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IQueryOEMWinSATCustomization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IQueryOEMWinSATCustomization {
    type Vtable = IQueryOEMWinSATCustomization_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc9a6a9f_ad4e_420e_9953_b34671e9df22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOEMWinSATCustomization_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IQueryRecentWinSATAssessment(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IQueryRecentWinSATAssessment {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn XML<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xpath: Param0, namespaces: Param1) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).XML)(::core::mem::transmute_copy(self), xpath.into_param().abi(), namespaces.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Info(&self) -> ::windows::core::Result<IProvideWinSATResultsInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Info)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IProvideWinSATResultsInfo>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IQueryRecentWinSATAssessment> for ::windows::core::IUnknown {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IQueryRecentWinSATAssessment> for ::windows::core::IUnknown {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IQueryRecentWinSATAssessment> for super::Com::IDispatch {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IQueryRecentWinSATAssessment> for super::Com::IDispatch {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &'a IQueryRecentWinSATAssessment {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IQueryRecentWinSATAssessment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IQueryRecentWinSATAssessment {
    type Vtable = IQueryRecentWinSATAssessment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8ad5d1f_3b47_4bdc_9375_7c6b1da4eca7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryRecentWinSATAssessment_Vtbl {
    pub base: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub XML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    XML: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwinsatassessmentinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Info: usize,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
pub struct IWinSATInitiateEvents(::windows::core::IUnknown);
impl IWinSATInitiateEvents {
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn WinSATComplete<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hresult: ::windows::core::HRESULT, strdescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WinSATComplete)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), strdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
    pub unsafe fn WinSATUpdate<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WinSATUpdate)(::core::mem::transmute_copy(self), ::core::mem::transmute(ucurrenttick), ::core::mem::transmute(uticktotal), strcurrentstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWinSATInitiateEvents> for ::windows::core::IUnknown {
    fn from(value: IWinSATInitiateEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinSATInitiateEvents> for ::windows::core::IUnknown {
    fn from(value: &IWinSATInitiateEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinSATInitiateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinSATInitiateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWinSATInitiateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::Interface for IWinSATInitiateEvents {
    type Vtable = IWinSATInitiateEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x262a1918_ba0d_41d5_92c2_fab4633ee74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinSATInitiateEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub WinSATComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, strdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
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
unsafe impl ::windows::core::Abi for WINSAT_ASSESSMENT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
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
unsafe impl ::windows::core::Abi for WINSAT_ASSESSMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_ASSESSMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_ASSESSMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
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
unsafe impl ::windows::core::Abi for WINSAT_BITMAP_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_BITMAP_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_BITMAP_SIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINSAT_OEM_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(3i32);
impl ::core::marker::Copy for WINSAT_OEM_DATA_TYPE {}
impl ::core::clone::Clone for WINSAT_OEM_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSAT_OEM_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINSAT_OEM_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINSAT_OEM_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSAT_OEM_DATA_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
