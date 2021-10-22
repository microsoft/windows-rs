#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const CAccessiblityWinSAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1847130566,
    41963,
    18778,
    [137, 183, 149, 100, 130, 225, 159, 122],
);
pub const CInitiateWinSAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1217606108,
    62944,
    17704,
    [159, 218, 69, 51, 27, 244, 165, 113],
);
pub const CProvideWinSATVisuals: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2671213950,
    58705,
    17656,
    [159, 148, 157, 179, 146, 176, 59, 123],
);
pub const CQueryAllWinSAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    98536723,
    50005,
    18420,
    [161, 30, 133, 27, 51, 140, 239, 184],
);
pub const CQueryOEMWinSATCustomization: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3296346551,
        46889,
        16975,
        [154, 249, 92, 179, 147, 79, 45, 250],
    );
pub const CQueryWinSAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4089314003,
    62070,
    18921,
    [155, 23, 196, 116, 244, 143, 7, 100],
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAccessibleWinSAT(::windows::runtime::IUnknown);
impl IAccessibleWinSAT {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::OleAutomation::ITypeInfo> {
        let mut result__: <super::OleAutomation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::OleAutomation::VARIANT,
        pexcepinfo: *mut super::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn accParent(&self) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
    pub unsafe fn accChildCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accChild<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accRole<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accHelp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accHelpTopic<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        pszhelpfile: *mut super::super::Foundation::BSTR,
        varchild: Param1,
        pidtopic: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pszhelpfile),
            varchild.into_param().abi(),
            ::std::mem::transmute(pidtopic),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accKeyboardShortcut<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accFocus(&self) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accSelection(&self) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accDefaultAction<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accSelect<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        flagsselect: i32,
        varchild: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(flagsselect),
            varchild.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accLocation<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        pxleft: *mut i32,
        pytop: *mut i32,
        pcxwidth: *mut i32,
        pcyheight: *mut i32,
        varchild: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pxleft),
            ::std::mem::transmute(pytop),
            ::std::mem::transmute(pcxwidth),
            ::std::mem::transmute(pcyheight),
            varchild.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accNavigate<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        navdir: i32,
        varstart: Param1,
    ) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(navdir),
            varstart.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accHitTest(
        &self,
        xleft: i32,
        ytop: i32,
    ) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(xleft),
            ::std::mem::transmute(ytop),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn accDoDefaultAction<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        varchild: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetaccName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        varchild: Param0,
        szname: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            szname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetaccValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        varchild: Param0,
        szvalue: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            varchild.into_param().abi(),
            szvalue.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccessiblityData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wsname: Param0,
        wsvalue: Param1,
        wsdesc: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            wsname.into_param().abi(),
            wsvalue.into_param().abi(),
            wsdesc.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAccessibleWinSAT {
    type Vtable = IAccessibleWinSAT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        820380042,
        38056,
        20472,
        [166, 154, 113, 182, 116, 19, 240, 123],
    );
}
impl ::std::convert::From<IAccessibleWinSAT> for ::windows::runtime::IUnknown {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAccessibleWinSAT> for ::windows::runtime::IUnknown {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAccessibleWinSAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_UI_Accessibility")]
impl ::std::convert::From<IAccessibleWinSAT> for super::super::UI::Accessibility::IAccessible {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Accessibility")]
impl ::std::convert::From<&IAccessibleWinSAT> for super::super::UI::Accessibility::IAccessible {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Accessibility")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::UI::Accessibility::IAccessible>
    for IAccessibleWinSAT
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::UI::Accessibility::IAccessible> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::UI::Accessibility::IAccessible,
        >::into(self))
    }
}
#[cfg(feature = "Win32_UI_Accessibility")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::UI::Accessibility::IAccessible>
    for &IAccessibleWinSAT
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::UI::Accessibility::IAccessible> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::UI::Accessibility::IAccessible,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IAccessibleWinSAT> for super::OleAutomation::IDispatch {
    fn from(value: IAccessibleWinSAT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IAccessibleWinSAT> for super::OleAutomation::IDispatch {
    fn from(value: &IAccessibleWinSAT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for IAccessibleWinSAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &IAccessibleWinSAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWinSAT_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdispparent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcountchildren: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        ppdispchild: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pvarrole: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pvarstate: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszhelp: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pidtopic: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszkeyboardshortcut: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarchild: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarchildren: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pszdefaultaction: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        flagsselect: i32,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pxleft: *mut i32,
        pytop: *mut i32,
        pcxwidth: *mut i32,
        pcyheight: *mut i32,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        navdir: i32,
        varstart: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pvarendupat: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        xleft: i32,
        ytop: i32,
        pvarchild: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        szname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varchild: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        szvalue: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wsname: super::super::Foundation::PWSTR,
        wsvalue: super::super::Foundation::PWSTR,
        wsdesc: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IInitiateWinSATAssessment(::windows::runtime::IUnknown);
impl IInitiateWinSATAssessment {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitiateAssessment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, IWinSATInitiateEvents>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        cmdline: Param0,
        pcallbacks: Param1,
        callerhwnd: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            cmdline.into_param().abi(),
            pcallbacks.into_param().abi(),
            callerhwnd.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitiateFormalAssessment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IWinSATInitiateEvents>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        pcallbacks: Param0,
        callerhwnd: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pcallbacks.into_param().abi(),
            callerhwnd.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CancelAssessment(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInitiateWinSATAssessment {
    type Vtable = IInitiateWinSATAssessment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3649305680,
        62911,
        18901,
        [181, 237, 204, 203, 24, 170, 127, 193],
    );
}
impl ::std::convert::From<IInitiateWinSATAssessment> for ::windows::runtime::IUnknown {
    fn from(value: IInitiateWinSATAssessment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInitiateWinSATAssessment> for ::windows::runtime::IUnknown {
    fn from(value: &IInitiateWinSATAssessment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IInitiateWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IInitiateWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitiateWinSATAssessment_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cmdline: super::super::Foundation::PWSTR,
        pcallbacks: ::windows::runtime::RawPtr,
        callerhwnd: super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcallbacks: ::windows::runtime::RawPtr,
        callerhwnd: super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProvideWinSATAssessmentInfo(::windows::runtime::IUnknown);
impl IProvideWinSATAssessmentInfo {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::OleAutomation::ITypeInfo> {
        let mut result__: <super::OleAutomation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::OleAutomation::VARIANT,
        pexcepinfo: *mut super::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Score(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProvideWinSATAssessmentInfo {
    type Vtable = IProvideWinSATAssessmentInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        215073664,
        21203,
        18040,
        [172, 111, 233, 41, 228, 128, 190, 158],
    );
}
impl ::std::convert::From<IProvideWinSATAssessmentInfo> for ::windows::runtime::IUnknown {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideWinSATAssessmentInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProvideWinSATAssessmentInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProvideWinSATAssessmentInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IProvideWinSATAssessmentInfo> for super::OleAutomation::IDispatch {
    fn from(value: IProvideWinSATAssessmentInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IProvideWinSATAssessmentInfo> for super::OleAutomation::IDispatch {
    fn from(value: &IProvideWinSATAssessmentInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for IProvideWinSATAssessmentInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for &IProvideWinSATAssessmentInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATAssessmentInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        score: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        title: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProvideWinSATResultsInfo(::windows::runtime::IUnknown);
impl IProvideWinSATResultsInfo {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::OleAutomation::ITypeInfo> {
        let mut result__: <super::OleAutomation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::OleAutomation::VARIANT,
        pexcepinfo: *mut super::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn GetAssessmentInfo(
        &self,
        assessment: WINSAT_ASSESSMENT_TYPE,
    ) -> ::windows::runtime::Result<IProvideWinSATAssessmentInfo> {
        let mut result__: <IProvideWinSATAssessmentInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(assessment),
            &mut result__,
        )
        .from_abi::<IProvideWinSATAssessmentInfo>(result__)
    }
    pub unsafe fn AssessmentState(&self) -> ::windows::runtime::Result<WINSAT_ASSESSMENT_STATE> {
        let mut result__: <WINSAT_ASSESSMENT_STATE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<WINSAT_ASSESSMENT_STATE>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn AssessmentDateTime(
        &self,
    ) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::VARIANT>(result__)
    }
    pub unsafe fn SystemRating(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RatingStateDesc(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProvideWinSATResultsInfo {
    type Vtable = IProvideWinSATResultsInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4164111709,
        22158,
        16501,
        [135, 95, 157, 243, 65, 80, 102, 64],
    );
}
impl ::std::convert::From<IProvideWinSATResultsInfo> for ::windows::runtime::IUnknown {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideWinSATResultsInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProvideWinSATResultsInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProvideWinSATResultsInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IProvideWinSATResultsInfo> for super::OleAutomation::IDispatch {
    fn from(value: IProvideWinSATResultsInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IProvideWinSATResultsInfo> for super::OleAutomation::IDispatch {
    fn from(value: &IProvideWinSATResultsInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for IProvideWinSATResultsInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for &IProvideWinSATResultsInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATResultsInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        assessment: WINSAT_ASSESSMENT_TYPE,
        ppinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        state: *mut WINSAT_ASSESSMENT_STATE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filetime: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        level: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProvideWinSATVisuals(::windows::runtime::IUnknown);
impl IProvideWinSATVisuals {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Bitmap(
        &self,
        bitmapsize: WINSAT_BITMAP_SIZE,
        state: WINSAT_ASSESSMENT_STATE,
        rating: f32,
    ) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: <super::super::Graphics::Gdi::HBITMAP as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bitmapsize),
            ::std::mem::transmute(state),
            ::std::mem::transmute(rating),
            &mut result__,
        )
        .from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProvideWinSATVisuals {
    type Vtable = IProvideWinSATVisuals_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2851384800,
        34586,
        17059,
        [184, 19, 48, 120, 210, 81, 98, 201],
    );
}
impl ::std::convert::From<IProvideWinSATVisuals> for ::windows::runtime::IUnknown {
    fn from(value: IProvideWinSATVisuals) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideWinSATVisuals> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideWinSATVisuals) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProvideWinSATVisuals {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProvideWinSATVisuals
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideWinSATVisuals_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bitmapsize: WINSAT_BITMAP_SIZE,
        state: WINSAT_ASSESSMENT_STATE,
        rating: f32,
        pbitmap: *mut super::super::Graphics::Gdi::HBITMAP,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IQueryAllWinSATAssessments(::windows::runtime::IUnknown);
impl IQueryAllWinSATAssessments {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::OleAutomation::ITypeInfo> {
        let mut result__: <super::OleAutomation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::OleAutomation::VARIANT,
        pexcepinfo: *mut super::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub unsafe fn AllXML<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        xpath: Param0,
        namespaces: Param1,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__ : < super::super::Data::Xml::MsXml:: IXMLDOMNodeList as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            xpath.into_param().abi(),
            namespaces.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IQueryAllWinSATAssessments {
    type Vtable = IQueryAllWinSATAssessments_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        193588509,
        25496,
        20458,
        [135, 252, 86, 125, 141, 25, 23, 111],
    );
}
impl ::std::convert::From<IQueryAllWinSATAssessments> for ::windows::runtime::IUnknown {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IQueryAllWinSATAssessments> for ::windows::runtime::IUnknown {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IQueryAllWinSATAssessments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IQueryAllWinSATAssessments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IQueryAllWinSATAssessments> for super::OleAutomation::IDispatch {
    fn from(value: IQueryAllWinSATAssessments) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IQueryAllWinSATAssessments> for super::OleAutomation::IDispatch {
    fn from(value: &IQueryAllWinSATAssessments) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for IQueryAllWinSATAssessments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for &IQueryAllWinSATAssessments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAllWinSATAssessments_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        xpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        namespaces: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppdomnodelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IQueryOEMWinSATCustomization(::windows::runtime::IUnknown);
impl IQueryOEMWinSATCustomization {
    pub unsafe fn GetOEMPrePopulationInfo(
        &self,
    ) -> ::windows::runtime::Result<WINSAT_OEM_DATA_TYPE> {
        let mut result__: <WINSAT_OEM_DATA_TYPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<WINSAT_OEM_DATA_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IQueryOEMWinSATCustomization {
    type Vtable = IQueryOEMWinSATCustomization_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3164236447,
        44366,
        16910,
        [153, 83, 179, 70, 113, 233, 223, 34],
    );
}
impl ::std::convert::From<IQueryOEMWinSATCustomization> for ::windows::runtime::IUnknown {
    fn from(value: IQueryOEMWinSATCustomization) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IQueryOEMWinSATCustomization> for ::windows::runtime::IUnknown {
    fn from(value: &IQueryOEMWinSATCustomization) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IQueryOEMWinSATCustomization
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IQueryOEMWinSATCustomization
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOEMWinSATCustomization_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        state: *mut WINSAT_OEM_DATA_TYPE,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IQueryRecentWinSATAssessment(::windows::runtime::IUnknown);
impl IQueryRecentWinSATAssessment {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::OleAutomation::ITypeInfo> {
        let mut result__: <super::OleAutomation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::OleAutomation::VARIANT,
        pexcepinfo: *mut super::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub unsafe fn XML<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        xpath: Param0,
        namespaces: Param1,
    ) -> ::windows::runtime::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__ : < super::super::Data::Xml::MsXml:: IXMLDOMNodeList as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            xpath.into_param().abi(),
            namespaces.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
    pub unsafe fn Info(&self) -> ::windows::runtime::Result<IProvideWinSATResultsInfo> {
        let mut result__: <IProvideWinSATResultsInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IProvideWinSATResultsInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IQueryRecentWinSATAssessment {
    type Vtable = IQueryRecentWinSATAssessment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4172111135,
        15175,
        19420,
        [147, 117, 124, 107, 29, 164, 236, 167],
    );
}
impl ::std::convert::From<IQueryRecentWinSATAssessment> for ::windows::runtime::IUnknown {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IQueryRecentWinSATAssessment> for ::windows::runtime::IUnknown {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IQueryRecentWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IQueryRecentWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IQueryRecentWinSATAssessment> for super::OleAutomation::IDispatch {
    fn from(value: IQueryRecentWinSATAssessment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IQueryRecentWinSATAssessment> for super::OleAutomation::IDispatch {
    fn from(value: &IQueryRecentWinSATAssessment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for IQueryRecentWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch>
    for &IQueryRecentWinSATAssessment
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryRecentWinSATAssessment_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        xpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        namespaces: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppdomnodelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppwinsatassessmentinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWinSATInitiateEvents(::windows::runtime::IUnknown);
impl IWinSATInitiateEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WinSATComplete<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hresult: ::windows::runtime::HRESULT,
        strdescription: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hresult),
            strdescription.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WinSATUpdate<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        ucurrenttick: u32,
        uticktotal: u32,
        strcurrentstate: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ucurrenttick),
            ::std::mem::transmute(uticktotal),
            strcurrentstate.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWinSATInitiateEvents {
    type Vtable = IWinSATInitiateEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        640293144,
        47629,
        16853,
        [146, 194, 250, 180, 99, 62, 231, 79],
    );
}
impl ::std::convert::From<IWinSATInitiateEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWinSATInitiateEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWinSATInitiateEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWinSATInitiateEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWinSATInitiateEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWinSATInitiateEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinSATInitiateEvents_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresult: ::windows::runtime::HRESULT,
        strdescription: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ucurrenttick: u32,
        uticktotal: u32,
        strcurrentstate: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINSAT_ASSESSMENT_STATE(pub i32);
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(0i32);
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(1i32);
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE =
    WINSAT_ASSESSMENT_STATE(2i32);
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE =
    WINSAT_ASSESSMENT_STATE(3i32);
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = WINSAT_ASSESSMENT_STATE(4i32);
impl ::std::convert::From<i32> for WINSAT_ASSESSMENT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINSAT_ASSESSMENT_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINSAT_ASSESSMENT_TYPE(pub i32);
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(0i32);
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(1i32);
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(2i32);
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(3i32);
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = WINSAT_ASSESSMENT_TYPE(4i32);
impl ::std::convert::From<i32> for WINSAT_ASSESSMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINSAT_ASSESSMENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINSAT_BITMAP_SIZE(pub i32);
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(0i32);
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = WINSAT_BITMAP_SIZE(1i32);
impl ::std::convert::From<i32> for WINSAT_BITMAP_SIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINSAT_BITMAP_SIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINSAT_OEM_DATA_TYPE(pub i32);
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(0i32);
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(1i32);
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(2i32);
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = WINSAT_OEM_DATA_TYPE(3i32);
impl ::std::convert::From<i32> for WINSAT_OEM_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINSAT_OEM_DATA_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
