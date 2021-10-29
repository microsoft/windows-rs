#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FOREIGN_STATUS(pub i32);
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
impl ::std::convert::From<i32> for FOREIGN_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FOREIGN_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQApplication(::windows::runtime::IUnknown);
impl IMSMQApplication {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        machinename: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            machinename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQApplication {
    type Vtable = IMSMQApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183621,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQApplication> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQApplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQApplication> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQApplication> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQApplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQApplication
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machinename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrguid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMSMQApplication2(::windows::runtime::IUnknown);
impl IMSMQApplication2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        machinename: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            machinename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn RegisterCertificate(
        &self,
        flags: *const super::Com::VARIANT,
        externalcertificate: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(externalcertificate),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQApplication2 {
    type Vtable = IMSMQApplication2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        312674560,
        29440,
        4562,
        [176, 230, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQApplication2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQApplication2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQApplication2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQApplication2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQApplication2> for IMSMQApplication {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication2> for IMSMQApplication {
    fn from(value: &IMSMQApplication2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication> for IMSMQApplication2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication> for &IMSMQApplication2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQApplication2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQApplication2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQApplication2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQApplication2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQApplication2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machinename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrguid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        externalcertificate: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrmachinename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionmajor: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionminor: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionbuild: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisdsenabled: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQApplication3(::windows::runtime::IUnknown);
impl IMSMQApplication3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        machinename: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            machinename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn RegisterCertificate(
        &self,
        flags: *const super::Com::VARIANT,
        externalcertificate: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(externalcertificate),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ActiveQueues(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PrivateQueues(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DirectoryServiceServer(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn BytesInAllQueues(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMachine<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrmachine: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            bstrmachine.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Connect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Tidy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQApplication3 {
    type Vtable = IMSMQApplication3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748767,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQApplication3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQApplication3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: &IMSMQApplication3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication2> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication2> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IMSMQApplication3> for IMSMQApplication {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQApplication3> for IMSMQApplication {
    fn from(value: &IMSMQApplication3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQApplication> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQApplication> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQApplication>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQApplication3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQApplication3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQApplication3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQApplication3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQApplication3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machinename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrguid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        externalcertificate: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrmachinename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionmajor: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionminor: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psmsmqversionbuild: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisdsenabled: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvactivequeues: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvprivatequeues: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdirectoryserviceserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisconnected: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvbytesinallqueues: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrmachine: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQCollection(::windows::runtime::IUnknown);
impl IMSMQCollection {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Item(
        &self,
        index: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQCollection {
    type Vtable = IMSMQCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        25734191,
        60595,
        16755,
        [151, 121, 99, 92, 162, 3, 156, 114],
    );
}
impl ::std::convert::From<IMSMQCollection> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQCollection> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQCollection> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCollection_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pvarret: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcount: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppunk: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQCoordinatedTransactionDispenser(::windows::runtime::IUnknown);
impl IMSMQCoordinatedTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction> {
        let mut result__: <IMSMQTransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQCoordinatedTransactionDispenser {
    type Vtable = IMSMQCoordinatedTransactionDispenser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183617,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQCoordinatedTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQCoordinatedTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser>
    for super::Ole::Automation::IDispatch
{
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser>
    for super::Ole::Automation::IDispatch
{
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQCoordinatedTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQCoordinatedTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQCoordinatedTransactionDispenser2(::windows::runtime::IUnknown);
impl IMSMQCoordinatedTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction2> {
        let mut result__: <IMSMQTransaction2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQCoordinatedTransactionDispenser2 {
    type Vtable = IMSMQCoordinatedTransactionDispenser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748752,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQCoordinatedTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQCoordinatedTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser2>
    for super::Ole::Automation::IDispatch
{
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser2>
    for super::Ole::Automation::IDispatch
{
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQCoordinatedTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQCoordinatedTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQCoordinatedTransactionDispenser3(::windows::runtime::IUnknown);
impl IMSMQCoordinatedTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction3> {
        let mut result__: <IMSMQTransaction3 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQCoordinatedTransactionDispenser3 {
    type Vtable = IMSMQCoordinatedTransactionDispenser3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748756,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQCoordinatedTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQCoordinatedTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQCoordinatedTransactionDispenser3>
    for super::Ole::Automation::IDispatch
{
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQCoordinatedTransactionDispenser3>
    for super::Ole::Automation::IDispatch
{
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQCoordinatedTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQCoordinatedTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQDestination(::windows::runtime::IUnknown);
impl IMSMQDestination {
    pub unsafe fn Open(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn IADs(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_IADs<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        piads: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            piads.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetADsPath<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstradspath: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstradspath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrpathname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrpathname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrformatname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            bstrformatname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Destinations(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_Destinations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        pdestinations: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pdestinations.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQDestination {
    type Vtable = IMSMQDestination_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748758,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQDestination> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQDestination> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQDestination) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQDestination {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQDestination {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQDestination> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQDestination> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQDestination) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQDestination {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQDestination
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQDestination_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppiads: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piads: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstradspath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstradspath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrpathname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrformatname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdestinations: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinations: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQEvent(::windows::runtime::IUnknown);
impl IMSMQEvent {}
unsafe impl ::windows::runtime::Interface for IMSMQEvent {
    type Vtable = IMSMQEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183607,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQEvent> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQEvent> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQEvent> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQEvent2(::windows::runtime::IUnknown);
impl IMSMQEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQEvent2 {
    type Vtable = IMSMQEvent2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748754,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQEvent2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQEvent2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQEvent2> for IMSMQEvent {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent2> for IMSMQEvent {
    fn from(value: &IMSMQEvent2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent> for &IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQEvent2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQEvent2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQEvent2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQEvent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQEvent3(::windows::runtime::IUnknown);
impl IMSMQEvent3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQEvent3 {
    type Vtable = IMSMQEvent3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748764,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQEvent3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQEvent3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: &IMSMQEvent3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent2> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent2> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IMSMQEvent3> for IMSMQEvent {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQEvent3> for IMSMQEvent {
    fn from(value: &IMSMQEvent3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQEvent> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQEvent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQEvent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQEvent3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQEvent3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQEvent3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQManagement(::windows::runtime::IUnknown);
impl IMSMQManagement {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Init(
        &self,
        machine: *const super::Com::VARIANT,
        pathname: *const super::Com::VARIANT,
        formatname: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(machine),
            ::std::mem::transmute(pathname),
            ::std::mem::transmute(formatname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQManagement {
    type Vtable = IMSMQManagement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3193897537,
        58505,
        18775,
        [140, 196, 164, 82, 252, 243, 226, 62],
    );
}
impl ::std::convert::From<IMSMQManagement> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQManagement> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQManagement> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQManagement> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQManagement_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machine: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pathname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        formatname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmessagecount: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plforeignstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plqueuetype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfislocal: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransactionalstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvbytesinqueue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQMessage(::windows::runtime::IUnknown);
impl IMSMQMessage {
    pub unsafe fn Class(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthlevel),
        )
        .ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ldelivery),
        )
        .ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ltrace),
        )
        .ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpriority),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lappspecific),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        varbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            varbody.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CorrelationId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetCorrelationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varmsgid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            varmsgid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lack),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(
        &self,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreachqueue),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(
        &self,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreceive),
        )
        .ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lhashalg),
        )
        .ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lencryptalg),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SentTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderCertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsendercert: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(
            ::std::mem::transmute_copy(self),
            varsendercert.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lsenderidtype),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Send<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQQueue>>(
        &self,
        destinationqueue: Param0,
        transaction: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(
            ::std::mem::transmute_copy(self),
            destinationqueue.into_param().abi(),
            ::std::mem::transmute(transaction),
        )
        .ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQMessage {
    type Vtable = IMSMQMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183604,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQMessage> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQMessage> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQMessage> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pldelivery: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ldelivery: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltrace: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ltrace: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plpriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpriority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plappspecific: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lappspecific: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidsrcmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbbody: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarbody: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varbody: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varmsgid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plack: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lack: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreachqueue: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreceive: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhashalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lhashalg: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plencryptalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lencryptalg: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenttime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plarrivedtime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfodest: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsendercert: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsendercert: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenderid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderidtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lsenderidtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destinationqueue: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQMessage2(::windows::runtime::IUnknown);
impl IMSMQMessage2 {
    pub unsafe fn Class(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthlevel),
        )
        .ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ldelivery),
        )
        .ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ltrace),
        )
        .ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpriority),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lappspecific),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        varbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            varbody.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CorrelationId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetCorrelationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varmsgid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            varmsgid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lack),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(
        &self,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreachqueue),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(
        &self,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreceive),
        )
        .ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lhashalg),
        )
        .ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lencryptalg),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SentTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderCertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsendercert: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(
            ::std::mem::transmute_copy(self),
            varsendercert.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lsenderidtype),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Send<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQQueue2>>(
        &self,
        destinationqueue: Param0,
        transaction: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(
            ::std::mem::transmute_copy(self),
            destinationqueue.into_param().abi(),
            ::std::mem::transmute(transaction),
        )
        .ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Extension(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetExtension<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varextension: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(
            ::std::mem::transmute_copy(self),
            varextension.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidconnectortype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(
            ::std::mem::transmute_copy(self),
            bstrguidconnectortype.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn DestinationSymmetricKey(
        &self,
    ) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetDestinationSymmetricKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        vardestsymmkey: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(
            ::std::mem::transmute_copy(self),
            vardestsymmkey.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Signature(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSignature<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsignature: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(
            ::std::mem::transmute_copy(self),
            varsignature.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(
        &self,
        lauthprovtype: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthprovtype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrauthprovname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(
            ::std::mem::transmute_copy(self),
            bstrauthprovname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsenderid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(
            ::std::mem::transmute_copy(self),
            varsenderid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmsgclass),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn TransactionId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).74)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).76)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQMessage2 {
    type Vtable = IMSMQMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3650304992,
        42343,
        4562,
        [176, 243, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQMessage2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQMessage2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQMessage2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQMessage2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pldelivery: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ldelivery: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltrace: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ltrace: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plpriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpriority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plappspecific: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lappspecific: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidsrcmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbbody: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarbody: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varbody: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varmsgid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plack: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lack: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreachqueue: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreceive: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhashalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lhashalg: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plencryptalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lencryptalg: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenttime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plarrivedtime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfodest: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsendercert: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsendercert: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenderid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderidtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lsenderidtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destinationqueue: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderversion: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarextension: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varextension: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidconnectortype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidconnectortype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoxactstatus: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvardestsymmkey: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vardestsymmkey: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsignature: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsignature: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthprovtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthprovtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrauthprovname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrauthprovname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsenderid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmsgclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmsgclass: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarxactid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisfirstinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pislastinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psreceivedauthenticationlevel: *mut i16,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQMessage3(::windows::runtime::IUnknown);
impl IMSMQMessage3 {
    pub unsafe fn Class(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthlevel),
        )
        .ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ldelivery),
        )
        .ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ltrace),
        )
        .ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpriority),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lappspecific),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        varbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            varbody.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CorrelationId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetCorrelationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varmsgid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            varmsgid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lack),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(
        &self,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreachqueue),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(
        &self,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreceive),
        )
        .ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lhashalg),
        )
        .ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lencryptalg),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SentTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderCertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsendercert: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(
            ::std::mem::transmute_copy(self),
            varsendercert.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lsenderidtype),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Send<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        destinationqueue: Param0,
        transaction: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(
            ::std::mem::transmute_copy(self),
            destinationqueue.into_param().abi(),
            ::std::mem::transmute(transaction),
        )
        .ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Extension(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetExtension<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varextension: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(
            ::std::mem::transmute_copy(self),
            varextension.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidconnectortype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(
            ::std::mem::transmute_copy(self),
            bstrguidconnectortype.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn DestinationSymmetricKey(
        &self,
    ) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetDestinationSymmetricKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        vardestsymmkey: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(
            ::std::mem::transmute_copy(self),
            vardestsymmkey.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Signature(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSignature<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsignature: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(
            ::std::mem::transmute_copy(self),
            varsignature.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(
        &self,
        lauthprovtype: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthprovtype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrauthprovname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(
            ::std::mem::transmute_copy(self),
            bstrauthprovname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsenderid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(
            ::std::mem::transmute_copy(self),
            varsenderid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmsgclass),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn TransactionId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).74)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).76)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).82)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo3>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).84)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo3>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn ResponseDestination(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).86)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_ResponseDestination<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        pdestresponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(
            ::std::mem::transmute_copy(self),
            pdestresponse.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Destination(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).88)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).90)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).91)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).92)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).93)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).94)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).95)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrsoapheader: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).96)(
            ::std::mem::transmute_copy(self),
            bstrsoapheader.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrsoapbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).97)(
            ::std::mem::transmute_copy(self),
            bstrsoapbody.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQMessage3 {
    type Vtable = IMSMQMessage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748762,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQMessage3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQMessage3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQMessage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQMessage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQMessage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQMessage3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQMessage3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQMessage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQMessage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQMessage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pldelivery: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ldelivery: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltrace: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ltrace: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plpriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpriority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plappspecific: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lappspecific: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidsrcmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbbody: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarbody: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varbody: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varmsgid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plack: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lack: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreachqueue: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreceive: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhashalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lhashalg: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plencryptalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lencryptalg: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenttime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plarrivedtime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfodest: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsendercert: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsendercert: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenderid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderidtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lsenderidtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destinationqueue: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderversion: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarextension: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varextension: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidconnectortype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidconnectortype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoxactstatus: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvardestsymmkey: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vardestsymmkey: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsignature: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsignature: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthprovtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthprovtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrauthprovname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrauthprovname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsenderid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmsgclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmsgclass: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarxactid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisfirstinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pislastinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psreceivedauthenticationlevel: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdestresponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestresponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdestdestination: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarlookupid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisfirstinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pislastinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrsoapenvelope: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcompoundmessage: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrsoapheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrsoapbody: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMSMQMessage4(::windows::runtime::IUnknown);
impl IMSMQMessage4 {
    pub unsafe fn Class(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthlevel),
        )
        .ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ldelivery),
        )
        .ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ltrace),
        )
        .ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpriority),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lappspecific),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        varbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            varbody.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CorrelationId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetCorrelationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varmsgid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            varmsgid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lack),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(
        &self,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreachqueue),
        )
        .ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(
        &self,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmaxtimetoreceive),
        )
        .ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lhashalg),
        )
        .ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lencryptalg),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SentTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderCertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsendercert: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(
            ::std::mem::transmute_copy(self),
            varsendercert.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SenderId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lsenderidtype),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Send<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        destinationqueue: Param0,
        transaction: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(
            ::std::mem::transmute_copy(self),
            destinationqueue.into_param().abi(),
            ::std::mem::transmute(transaction),
        )
        .ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Extension(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetExtension<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varextension: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(
            ::std::mem::transmute_copy(self),
            varextension.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidconnectortype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(
            ::std::mem::transmute_copy(self),
            bstrguidconnectortype.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn DestinationSymmetricKey(
        &self,
    ) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetDestinationSymmetricKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        vardestsymmkey: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(
            ::std::mem::transmute_copy(self),
            vardestsymmkey.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Signature(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSignature<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsignature: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(
            ::std::mem::transmute_copy(self),
            varsignature.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(
        &self,
        lauthprovtype: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthprovtype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrauthprovname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(
            ::std::mem::transmute_copy(self),
            bstrauthprovname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSenderId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsenderid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(
            ::std::mem::transmute_copy(self),
            varsenderid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmsgclass),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn TransactionId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).74)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).76)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo2>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).82)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo4>,
    >(
        &self,
        pqinforesponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(
            ::std::mem::transmute_copy(self),
            pqinforesponse.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).84)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueueInfo4>,
    >(
        &self,
        pqinfoadmin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(
            ::std::mem::transmute_copy(self),
            pqinfoadmin.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn ResponseDestination(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).86)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_ResponseDestination<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>,
    >(
        &self,
        pdestresponse: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(
            ::std::mem::transmute_copy(self),
            pdestresponse.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Destination(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).88)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupId(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).90)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).91)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).92)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).93)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).94)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).95)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrsoapheader: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).96)(
            ::std::mem::transmute_copy(self),
            bstrsoapheader.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrsoapbody: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).97)(
            ::std::mem::transmute_copy(self),
            bstrsoapbody.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQMessage4 {
    type Vtable = IMSMQMessage4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748771,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQMessage4> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQMessage4> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQMessage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQMessage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQMessage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQMessage4> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQMessage4> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQMessage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQMessage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQMessage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage4_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pldelivery: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ldelivery: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltrace: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ltrace: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plpriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpriority: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plappspecific: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lappspecific: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidsrcmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbbody: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarbody: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varbody: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmsgid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varmsgid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plack: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lack: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreachqueue: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreachqueue: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmaxtimetoreceive: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmaxtimetoreceive: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhashalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lhashalg: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plencryptalg: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lencryptalg: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenttime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plarrivedtime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfodest: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsendercert: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsendercert: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsenderid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderidtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lsenderidtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destinationqueue: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsenderversion: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarextension: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varextension: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidconnectortype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidconnectortype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoxactstatus: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvardestsymmkey: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vardestsymmkey: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsignature: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsignature: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthprovtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthprovtype: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrauthprovname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrauthprovname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsenderid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmsgclass: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmsgclass: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarxactid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisfirstinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pislastinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psreceivedauthenticationlevel: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinforesponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinforesponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfoadmin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pqinfoadmin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdestresponse: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestresponse: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppdestdestination: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarlookupid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisauthenticated: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisfirstinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pislastinxact: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrsoapenvelope: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcompoundmessage: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrsoapheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrsoapbody: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMSMQOutgoingQueueManagement(::windows::runtime::IUnknown);
impl IMSMQOutgoingQueueManagement {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Init(
        &self,
        machine: *const super::Com::VARIANT,
        pathname: *const super::Com::VARIANT,
        formatname: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(machine),
            ::std::mem::transmute(pathname),
            ::std::mem::transmute(formatname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn NextHops(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn EodGetSendInfo(&self) -> ::windows::runtime::Result<IMSMQCollection> {
        let mut result__: <IMSMQCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQCollection>(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EodResend(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQOutgoingQueueManagement {
    type Vtable = IMSMQOutgoingQueueManagement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1690597627,
        63920,
        18069,
        [138, 127, 67, 154, 201, 67, 38, 211],
    );
}
impl ::std::convert::From<IMSMQOutgoingQueueManagement> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQOutgoingQueueManagement> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQOutgoingQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQOutgoingQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQManagement> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQManagement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQManagement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQManagement> for &IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQManagement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQManagement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQOutgoingQueueManagement> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQOutgoingQueueManagement> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQOutgoingQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQOutgoingQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQOutgoingQueueManagement_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machine: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pathname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        formatname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmessagecount: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plforeignstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plqueuetype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfislocal: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransactionalstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvbytesinqueue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plstate: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvnexthops: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcollection: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQPrivateDestination(::windows::runtime::IUnknown);
impl IMSMQPrivateDestination {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        varhandle: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            varhandle.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQPrivateDestination {
    type Vtable = IMSMQPrivateDestination_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748759,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQPrivateDestination> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQPrivateDestination> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQPrivateDestination
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQPrivateDestination
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQPrivateDestination> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQPrivateDestination> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQPrivateDestination
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQPrivateDestination
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateDestination_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarhandle: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varhandle: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQPrivateEvent(::windows::runtime::IUnknown);
impl IMSMQPrivateEvent {
    pub unsafe fn Hwnd(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn FireArrivedEvent<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQQueue>>(
        &self,
        pq: Param0,
        msgcursor: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            pq.into_param().abi(),
            ::std::mem::transmute(msgcursor),
        )
        .ok()
    }
    pub unsafe fn FireArrivedErrorEvent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IMSMQQueue>,
    >(
        &self,
        pq: Param0,
        hrstatus: ::windows::runtime::HRESULT,
        msgcursor: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            pq.into_param().abi(),
            ::std::mem::transmute(hrstatus),
            ::std::mem::transmute(msgcursor),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQPrivateEvent {
    type Vtable = IMSMQPrivateEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3618321217,
        51667,
        4561,
        [187, 71, 0, 128, 199, 197, 162, 192],
    );
}
impl ::std::convert::From<IMSMQPrivateEvent> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQPrivateEvent> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQPrivateEvent> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQPrivateEvent> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQPrivateEvent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQPrivateEvent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateEvent_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pq: ::windows::runtime::RawPtr,
        msgcursor: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pq: ::windows::runtime::RawPtr,
        hrstatus: ::windows::runtime::HRESULT,
        msgcursor: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQuery(::windows::runtime::IUnknown);
impl IMSMQQuery {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos> {
        let mut result__: <IMSMQQueueInfos as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQuery {
    type Vtable = IMSMQQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183602,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQQuery> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQuery> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQuery> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQuery> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQuery2(::windows::runtime::IUnknown);
impl IMSMQQuery2 {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos2> {
        let mut result__: <IMSMQQueueInfos2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos2>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQuery2 {
    type Vtable = IMSMQQuery2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748750,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQuery2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQuery2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQuery2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQuery2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQuery2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQuery2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQuery3(::windows::runtime::IUnknown);
impl IMSMQQuery3 {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue_v2(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos3> {
        let mut result__: <IMSMQQueueInfos3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
        multicastaddress: *const super::Com::VARIANT,
        relmulticastaddress: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos3> {
        let mut result__: <IMSMQQueueInfos3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            ::std::mem::transmute(multicastaddress),
            ::std::mem::transmute(relmulticastaddress),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQuery3 {
    type Vtable = IMSMQQuery3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748761,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQuery3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQuery3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQuery3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQuery3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQuery3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQuery3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        multicastaddress: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmulticastaddress: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQuery4(::windows::runtime::IUnknown);
impl IMSMQQuery4 {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue_v2(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos4> {
        let mut result__: <IMSMQQueueInfos4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
        multicastaddress: *const super::Com::VARIANT,
        relmulticastaddress: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQQueueInfos4> {
        let mut result__: <IMSMQQueueInfos4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(queueguid),
            ::std::mem::transmute(servicetypeguid),
            ::std::mem::transmute(label),
            ::std::mem::transmute(createtime),
            ::std::mem::transmute(modifytime),
            ::std::mem::transmute(relservicetype),
            ::std::mem::transmute(rellabel),
            ::std::mem::transmute(relcreatetime),
            ::std::mem::transmute(relmodifytime),
            ::std::mem::transmute(multicastaddress),
            ::std::mem::transmute(relmulticastaddress),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQuery4 {
    type Vtable = IMSMQQuery4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748772,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQuery4> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQuery4> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQuery4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQuery4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQuery4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQuery4> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQuery4> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQuery4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQuery4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQuery4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery4_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        queueguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        multicastaddress: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        relmulticastaddress: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueue(::windows::runtime::IUnknown);
impl IMSMQQueue {
    pub unsafe fn Access(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQEvent>>(
        &self,
        event: Param0,
        cursor: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            event.into_param().abi(),
            ::std::mem::transmute(cursor),
            ::std::mem::transmute(receivetimeout),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueue {
    type Vtable = IMSMQQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183606,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQQueue> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueue> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueue> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        placcess: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsharemode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhandle: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        event: ::windows::runtime::RawPtr,
        cursor: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueue2(::windows::runtime::IUnknown);
impl IMSMQQueue2 {
    pub unsafe fn Access(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQEvent2>>(
        &self,
        event: Param0,
        cursor: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            event.into_param().abi(),
            ::std::mem::transmute(cursor),
            ::std::mem::transmute(receivetimeout),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueue2 {
    type Vtable = IMSMQQueue2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4010112224,
        1752,
        4563,
        [177, 0, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueue2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueue2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueue2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueue2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueue2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueue2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueue2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueue2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueue2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueue2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        placcess: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsharemode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhandle: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        event: ::windows::runtime::RawPtr,
        cursor: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueue3(::windows::runtime::IUnknown);
impl IMSMQQueue3 {
    pub unsafe fn Access(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQEvent3>>(
        &self,
        event: Param0,
        cursor: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            event.into_param().abi(),
            ::std::mem::transmute(cursor),
            ::std::mem::transmute(receivetimeout),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Handle2(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveNextByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceivePreviousByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveFirstByLookupId(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveLastByLookupId(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNextByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekPreviousByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekFirstByLookupId(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekLastByLookupId(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage3>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueue3 {
    type Vtable = IMSMQQueue3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748763,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueue3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueue3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueue3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueue3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueue3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueue3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueue3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueue3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueue3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueue3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        placcess: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsharemode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhandle: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        event: ::windows::runtime::RawPtr,
        cursor: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarhandle: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueue4(::windows::runtime::IUnknown);
impl IMSMQQueue4 {
    pub unsafe fn Access(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IMSMQEvent3>>(
        &self,
        event: Param0,
        cursor: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            event.into_param().abi(),
            ::std::mem::transmute(cursor),
            ::std::mem::transmute(receivetimeout),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent_v1(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent_v1(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            &mut result__,
        )
        .from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Receive(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Peek(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveCurrent(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNext(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekCurrent(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        receivetimeout: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(receivetimeout),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Handle2(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveNextByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceivePreviousByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveFirstByLookupId(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveLastByLookupId(
        &self,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekNextByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekPreviousByLookupId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekFirstByLookupId(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn PeekLastByLookupId(
        &self,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReceiveByLookupIdAllowPeek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        lookupid: Param0,
        transaction: *const super::Com::VARIANT,
        wantdestinationqueue: *const super::Com::VARIANT,
        wantbody: *const super::Com::VARIANT,
        wantconnectortype: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            lookupid.into_param().abi(),
            ::std::mem::transmute(transaction),
            ::std::mem::transmute(wantdestinationqueue),
            ::std::mem::transmute(wantbody),
            ::std::mem::transmute(wantconnectortype),
            &mut result__,
        )
        .from_abi::<IMSMQMessage4>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueue4 {
    type Vtable = IMSMQQueue4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748768,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueue4> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueue4> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueue4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueue4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueue4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueue4> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueue4> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueue4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueue4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueue4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue4_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        placcess: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plsharemode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plhandle: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        event: ::windows::runtime::RawPtr,
        cursor: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        receivetimeout: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarhandle: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisopen: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lookupid: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        transaction: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantdestinationqueue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantbody: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        wantconnectortype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        ppmsg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfo(::windows::runtime::IUnknown);
impl IMSMQQueueInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidservicetype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguidservicetype.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrpathname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstrpathname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrformatname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrformatname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lquota),
        )
        .ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lbasepriority),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CreateTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ModifyTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthenticate),
        )
        .ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournalquota),
        )
        .ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Create(
        &self,
        istransactional: *const super::Com::VARIANT,
        isworldreadable: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(istransactional),
            ::std::mem::transmute(isworldreadable),
        )
        .ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(
        &self,
        access: i32,
        sharemode: i32,
    ) -> ::windows::runtime::Result<IMSMQQueue> {
        let mut result__: <IMSMQQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(access),
            ::std::mem::transmute(sharemode),
            &mut result__,
        )
        .from_abi::<IMSMQQueue>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfo {
    type Vtable = IMSMQQueueInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183611,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQQueueInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfo> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfo> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueueInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidqueue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidservicetype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidservicetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrpathname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrformatname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plbasepriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lbasepriority: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcreatetime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmodifytime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthenticate: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthenticate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournalquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournalquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        istransactional: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        isworldreadable: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        access: i32,
        sharemode: i32,
        ppq: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfo2(::windows::runtime::IUnknown);
impl IMSMQQueueInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidservicetype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguidservicetype.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrpathname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstrpathname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrformatname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrformatname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lquota),
        )
        .ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lbasepriority),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CreateTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ModifyTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthenticate),
        )
        .ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournalquota),
        )
        .ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Create(
        &self,
        istransactional: *const super::Com::VARIANT,
        isworldreadable: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(istransactional),
            ::std::mem::transmute(isworldreadable),
        )
        .ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(
        &self,
        access: i32,
        sharemode: i32,
    ) -> ::windows::runtime::Result<IMSMQQueue2> {
        let mut result__: <IMSMQQueue2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(access),
            ::std::mem::transmute(sharemode),
            &mut result__,
        )
        .from_abi::<IMSMQQueue2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSecurity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsecurity: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            varsecurity.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfo2 {
    type Vtable = IMSMQQueueInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4246162048,
        35279,
        4562,
        [176, 242, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfo2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfo2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidqueue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidservicetype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidservicetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrpathname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrformatname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plbasepriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lbasepriority: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcreatetime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmodifytime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthenticate: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthenticate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournalquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournalquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        istransactional: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        isworldreadable: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        access: i32,
        sharemode: i32,
        ppq: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathnamedns: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsecurity: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsecurity: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfo3(::windows::runtime::IUnknown);
impl IMSMQQueueInfo3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidservicetype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguidservicetype.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrpathname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstrpathname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrformatname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrformatname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lquota),
        )
        .ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lbasepriority),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CreateTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ModifyTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthenticate),
        )
        .ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournalquota),
        )
        .ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Create(
        &self,
        istransactional: *const super::Com::VARIANT,
        isworldreadable: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(istransactional),
            ::std::mem::transmute(isworldreadable),
        )
        .ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(
        &self,
        access: i32,
        sharemode: i32,
    ) -> ::windows::runtime::Result<IMSMQQueue3> {
        let mut result__: <IMSMQQueue3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(access),
            ::std::mem::transmute(sharemode),
            &mut result__,
        )
        .from_abi::<IMSMQQueue3>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSecurity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsecurity: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            varsecurity.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrmulticastaddress: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            bstrmulticastaddress.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfo3 {
    type Vtable = IMSMQQueueInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748765,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfo3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfo3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfo3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfo3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidqueue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidservicetype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidservicetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrpathname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrformatname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plbasepriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lbasepriority: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcreatetime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmodifytime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthenticate: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthenticate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournalquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournalquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        istransactional: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        isworldreadable: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        access: i32,
        sharemode: i32,
        ppq: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathnamedns: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsecurity: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsecurity: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmulticastaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrmulticastaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstradspath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMSMQQueueInfo4(::windows::runtime::IUnknown);
impl IMSMQQueueInfo4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrguidservicetype: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrguidservicetype.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrlabel: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrlabel.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrpathname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstrpathname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrformatname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrformatname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lprivlevel),
        )
        .ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournal),
        )
        .ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lquota),
        )
        .ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lbasepriority),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn CreateTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ModifyTime(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lauthenticate),
        )
        .ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ljournalquota),
        )
        .ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Create(
        &self,
        istransactional: *const super::Com::VARIANT,
        isworldreadable: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(istransactional),
            ::std::mem::transmute(isworldreadable),
        )
        .ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(
        &self,
        access: i32,
        sharemode: i32,
    ) -> ::windows::runtime::Result<IMSMQQueue4> {
        let mut result__: <IMSMQQueue4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(access),
            ::std::mem::transmute(sharemode),
            &mut result__,
        )
        .from_abi::<IMSMQQueue4>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn SetSecurity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>,
    >(
        &self,
        varsecurity: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            varsecurity.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrmulticastaddress: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            bstrmulticastaddress.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfo4 {
    type Vtable = IMSMQQueueInfo4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748769,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfo4> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfo4> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfo4> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfo4> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo4_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidqueue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrguidservicetype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrguidservicetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrlabel: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrlabel: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrpathname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrformatname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plprivlevel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lprivlevel: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournal: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournal: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plbasepriority: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lbasepriority: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarcreatetime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarmodifytime: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plauthenticate: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lauthenticate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournalquota: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ljournalquota: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        istransactional: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        isworldreadable: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        access: i32,
        sharemode: i32,
        ppq: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrpathnamedns: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvarsecurity: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varsecurity: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistransactional: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisworldreadable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmulticastaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrmulticastaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstradspath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMSMQQueueInfos(::windows::runtime::IUnknown);
impl IMSMQQueueInfos {
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfos {
    type Vtable = IMSMQQueueInfos_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183613,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQQueueInfos> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfos> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfos> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfos> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMSMQQueueInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfonext: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfos2(::windows::runtime::IUnknown);
impl IMSMQQueueInfos2 {
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfos2 {
    type Vtable = IMSMQQueueInfos2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748751,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfos2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfos2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfos2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfos2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQQueueInfos2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfonext: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfos3(::windows::runtime::IUnknown);
impl IMSMQQueueInfos3 {
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfos3 {
    type Vtable = IMSMQQueueInfos3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748766,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfos3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfos3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfos3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfos3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQQueueInfos3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfonext: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueInfos4(::windows::runtime::IUnknown);
impl IMSMQQueueInfos4 {
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueInfos4 {
    type Vtable = IMSMQQueueInfos4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748770,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQQueueInfos4> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueInfos4> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueInfos4> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueInfos4> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQQueueInfos4
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos4_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppqinfonext: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQQueueManagement(::windows::runtime::IUnknown);
impl IMSMQQueueManagement {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Init(
        &self,
        machine: *const super::Com::VARIANT,
        pathname: *const super::Com::VARIANT,
        formatname: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(machine),
            ::std::mem::transmute(pathname),
            ::std::mem::transmute(formatname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn JournalMessageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn BytesInJournal(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn EodGetReceiveInfo(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQQueueManagement {
    type Vtable = IMSMQQueueManagement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2143188825,
        22368,
        17485,
        [184, 165, 94, 122, 185, 168, 76, 206],
    );
}
impl ::std::convert::From<IMSMQQueueManagement> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueManagement> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQManagement> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQManagement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQManagement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQManagement> for &IMSMQQueueManagement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQManagement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQManagement>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQQueueManagement> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQQueueManagement> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQQueueManagement
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueManagement_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        machine: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pathname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        formatname: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrformatname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrmachine: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plmessagecount: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plforeignstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plqueuetype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfislocal: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransactionalstatus: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvbytesinqueue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pljournalmessagecount: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvbytesinjournal: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvcollection: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransaction(::windows::runtime::IUnknown);
impl IMSMQTransaction {
    pub unsafe fn Transaction(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Commit(
        &self,
        fretaining: *const super::Com::VARIANT,
        grftc: *const super::Com::VARIANT,
        grfrm: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(grftc),
            ::std::mem::transmute(grfrm),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Abort(
        &self,
        fretaining: *const super::Com::VARIANT,
        fasync: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(fasync),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransaction {
    type Vtable = IMSMQTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183615,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQTransaction> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransaction> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransaction> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMSMQTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransaction
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransaction: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grftc: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grfrm: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        fasync: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransaction2(::windows::runtime::IUnknown);
impl IMSMQTransaction2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    pub unsafe fn Transaction(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Commit(
        &self,
        fretaining: *const super::Com::VARIANT,
        grftc: *const super::Com::VARIANT,
        grfrm: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(grftc),
            ::std::mem::transmute(grfrm),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Abort(
        &self,
        fretaining: *const super::Com::VARIANT,
        fasync: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(fasync),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn InitNew<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        vartransaction: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            vartransaction.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransaction2 {
    type Vtable = IMSMQTransaction2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        752928176,
        28263,
        4562,
        [176, 230, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQTransaction2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQTransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction> for &IMSMQTransaction2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransaction2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransaction2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQTransaction2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransaction2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransaction: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grftc: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grfrm: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        fasync: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vartransaction: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransaction3(::windows::runtime::IUnknown);
impl IMSMQTransaction3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::ITypeInfo>(result__)
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut super::Com::VARIANT,
        pexcepinfo: *mut super::Ole::Automation::EXCEPINFO,
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
    pub unsafe fn Transaction(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Commit(
        &self,
        fretaining: *const super::Com::VARIANT,
        grftc: *const super::Com::VARIANT,
        grfrm: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(grftc),
            ::std::mem::transmute(grfrm),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Abort(
        &self,
        fretaining: *const super::Com::VARIANT,
        fasync: *const super::Com::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fretaining),
            ::std::mem::transmute(fasync),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn InitNew<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        vartransaction: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            vartransaction.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ITransaction(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransaction3 {
    type Vtable = IMSMQTransaction3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748755,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQTransaction3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction2> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction2> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSMQTransaction> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSMQTransaction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSMQTransaction>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransaction3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransaction3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQTransaction3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransaction3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pltransaction: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grftc: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        grfrm: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fretaining: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        fasync: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vartransaction: ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvaritransaction: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransactionDispenser(::windows::runtime::IUnknown);
impl IMSMQTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction> {
        let mut result__: <IMSMQTransaction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransactionDispenser {
    type Vtable = IMSMQTransactionDispenser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183619,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<IMSMQTransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransactionDispenser> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransactionDispenser> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransactionDispenser> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransactionDispenser
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransactionDispenser2(::windows::runtime::IUnknown);
impl IMSMQTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction2> {
        let mut result__: <IMSMQTransaction2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransactionDispenser2 {
    type Vtable = IMSMQTransactionDispenser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748753,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQTransactionDispenser2> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransactionDispenser2> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransactionDispenser2> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransactionDispenser2> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransactionDispenser2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser2_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSMQTransactionDispenser3(::windows::runtime::IUnknown);
impl IMSMQTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::runtime::Result<IMSMQTransaction3> {
        let mut result__: <IMSMQTransaction3 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMSMQTransactionDispenser3 {
    type Vtable = IMSMQTransactionDispenser3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953748757,
        8552,
        4563,
        [137, 140, 0, 224, 44, 7, 79, 107],
    );
}
impl ::std::convert::From<IMSMQTransactionDispenser3> for ::windows::runtime::IUnknown {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSMQTransactionDispenser3> for ::windows::runtime::IUnknown {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IMSMQTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IMSMQTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMSMQTransactionDispenser3> for super::Ole::Automation::IDispatch {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMSMQTransactionDispenser3> for super::Ole::Automation::IDispatch {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for IMSMQTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMSMQTransactionDispenser3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser3_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptransaction: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcolproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
pub const LONG_LIVED: u32 = 4294967294u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MQACCESS(pub i32);
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
impl ::std::convert::From<i32> for MQACCESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQACCESS {
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
pub struct MQAUTHENTICATE(pub i32);
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
impl ::std::convert::From<i32> for MQAUTHENTICATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQAUTHENTICATE {
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
pub struct MQCALG(pub i32);
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
impl ::std::convert::From<i32> for MQCALG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQCALG {
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
pub struct MQCERT_REGISTER(pub i32);
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
impl ::std::convert::From<i32> for MQCERT_REGISTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQCERT_REGISTER {
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
pub struct MQDEFAULT(pub i32);
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
impl ::std::convert::From<i32> for MQDEFAULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQDEFAULT {
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
pub struct MQERROR(pub i32);
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
impl ::std::convert::From<i32> for MQERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQERROR {
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
pub struct MQJOURNAL(pub i32);
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
impl ::std::convert::From<i32> for MQJOURNAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQJOURNAL {
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
pub struct MQMAX(pub i32);
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
impl ::std::convert::From<i32> for MQMAX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMAX {
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
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
impl ::std::convert::From<i32> for MQMSGACKNOWLEDGEMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGACKNOWLEDGEMENT {
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
pub struct MQMSGAUTHENTICATION(pub i32);
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
impl ::std::convert::From<i32> for MQMSGAUTHENTICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGAUTHENTICATION {
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
pub struct MQMSGAUTHLEVEL(pub i32);
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
impl ::std::convert::From<i32> for MQMSGAUTHLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGAUTHLEVEL {
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
pub struct MQMSGCLASS(pub i32);
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
impl ::std::convert::From<i32> for MQMSGCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGCLASS {
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
pub struct MQMSGCURSOR(pub i32);
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
impl ::std::convert::From<i32> for MQMSGCURSOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGCURSOR {
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
pub struct MQMSGDELIVERY(pub i32);
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
impl ::std::convert::From<i32> for MQMSGDELIVERY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGDELIVERY {
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
pub struct MQMSGIDSIZE(pub i32);
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
impl ::std::convert::From<i32> for MQMSGIDSIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGIDSIZE {
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
pub struct MQMSGJOURNAL(pub i32);
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
impl ::std::convert::From<i32> for MQMSGJOURNAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGJOURNAL {
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
pub struct MQMSGMAX(pub i32);
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
impl ::std::convert::From<i32> for MQMSGMAX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGMAX {
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
pub struct MQMSGPRIVLEVEL(pub i32);
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
impl ::std::convert::From<i32> for MQMSGPRIVLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGPRIVLEVEL {
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
pub struct MQMSGSENDERIDTYPE(pub i32);
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
impl ::std::convert::From<i32> for MQMSGSENDERIDTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGSENDERIDTYPE {
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
pub struct MQMSGTRACE(pub i32);
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
impl ::std::convert::From<i32> for MQMSGTRACE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQMSGTRACE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MQPRIORITY(pub i32);
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
impl ::std::convert::From<i32> for MQPRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQPRIORITY {
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
pub struct MQPRIVLEVEL(pub i32);
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
impl ::std::convert::From<i32> for MQPRIVLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQPRIVLEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MQSHARE(pub i32);
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
impl ::std::convert::From<i32> for MQSHARE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQSHARE {
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
pub struct MQTRANSACTION(pub i32);
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
impl ::std::convert::From<i32> for MQTRANSACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQTRANSACTION {
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
pub struct MQTRANSACTIONAL(pub i32);
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
impl ::std::convert::From<i32> for MQTRANSACTIONAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQTRANSACTIONAL {
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
pub struct MQWARNING(pub i32);
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
impl ::std::convert::From<i32> for MQWARNING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MQWARNING {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072824164i32 as _);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072824165i32 as _);
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072824167i32 as _);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072824166i32 as _);
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
pub const MQ_MOVE_ACCESS: u32 = 4u32;
pub const MQ_OK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(0i32 as _);
pub const MSMQApplication: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183622,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4146827313,
    12044,
    17384,
    [146, 78, 230, 5, 44, 220, 73, 63],
);
pub const MSMQCoordinatedTransactionDispenser: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3621183618,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
pub const MSMQDestination: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3953748760,
    8552,
    4563,
    [137, 140, 0, 224, 44, 7, 79, 107],
);
pub const MSMQEvent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183610,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQManagement: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    969840382,
    62661,
    17540,
    [161, 67, 76, 45, 93, 50, 66, 41],
);
pub const MSMQMessage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183605,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQOutgoingQueueManagement: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        25706524,
        9338,
        20461,
        [153, 198, 191, 20, 17, 157, 112, 85],
    );
pub const MSMQQuery: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183603,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183609,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQQueueInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183612,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQQueueInfos: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183614,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQQueueManagement: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    867618942,
    62077,
    17146,
    [178, 215, 191, 130, 225, 30, 147, 116],
);
pub const MSMQTransaction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3621183616,
    56525,
    4560,
    [170, 75, 0, 96, 151, 13, 235, 174],
);
pub const MSMQTransactionDispenser: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3621183620,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
pub const PREQ: u32 = 4u32;
pub const PRGE: u32 = 3u32;
pub const PRGT: u32 = 2u32;
pub const PRLE: u32 = 1u32;
pub const PRLT: u32 = 0u32;
pub const PRNE: u32 = 5u32;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
pub const PROPID_M_BASE: u32 = 0u32;
pub const PROPID_M_BODY: u32 = 9u32;
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
pub const PROPID_M_CLASS: u32 = 1u32;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
pub const PROPID_M_DELIVERY: u32 = 5u32;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
pub const PROPID_M_EXTENSION: u32 = 35u32;
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
pub const PROPID_M_HASH_ALG: u32 = 26u32;
pub const PROPID_M_JOURNAL: u32 = 7u32;
pub const PROPID_M_LABEL: u32 = 11u32;
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
pub const PROPID_M_LOOKUPID: u32 = 60u32;
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
pub const PROPID_M_MSGID: u32 = 2u32;
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
pub const PROPID_M_PRIORITY: u32 = 4u32;
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
pub const PROPID_M_PROV_NAME: u32 = 48u32;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
pub const PROPID_M_SENDERID: u32 = 20u32;
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
pub const PROPID_M_SENTTIME: u32 = 31u32;
pub const PROPID_M_SIGNATURE: u32 = 45u32;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
pub const PROPID_M_TRACE: u32 = 41u32;
pub const PROPID_M_VERSION: u32 = 19u32;
pub const PROPID_M_XACTID: u32 = 52u32;
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
pub const PROPID_PC_BASE: u32 = 5800u32;
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
pub const PROPID_PC_VERSION: u32 = 5801u32;
pub const PROPID_QM_BASE: u32 = 200u32;
pub const PROPID_QM_CONNECTION: u32 = 204u32;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
pub const PROPID_QM_PATHNAME: u32 = 203u32;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
pub const PROPID_QM_SITE_ID: u32 = 201u32;
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
pub const PROPID_Q_BASE: u32 = 100u32;
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
pub const PROPID_Q_INSTANCE: u32 = 101u32;
pub const PROPID_Q_JOURNAL: u32 = 104u32;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
pub const PROPID_Q_LABEL: u32 = 108u32;
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
pub const PROPID_Q_PATHNAME: u32 = 103u32;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
pub const PROPID_Q_QUOTA: u32 = 105u32;
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
pub const PROPID_Q_TYPE: u32 = 102u32;
pub const QUERY_SORTASCEND: u32 = 0u32;
pub const QUERY_SORTDESCEND: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct QUEUE_STATE(pub i32);
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
impl ::std::convert::From<i32> for QUEUE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QUEUE_STATE {
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
pub struct QUEUE_TYPE(pub i32);
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
impl ::std::convert::From<i32> for QUEUE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QUEUE_TYPE {
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
pub struct RELOPS(pub i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
impl ::std::convert::From<i32> for RELOPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RELOPS {
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
pub struct XACT_STATUS(pub i32);
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
impl ::std::convert::From<i32> for XACT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XACT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _DMSMQEventEvents(::windows::runtime::IUnknown);
impl _DMSMQEventEvents {}
unsafe impl ::windows::runtime::Interface for _DMSMQEventEvents {
    type Vtable = _DMSMQEventEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3621183608,
        56525,
        4560,
        [170, 75, 0, 96, 151, 13, 235, 174],
    );
}
impl ::std::convert::From<_DMSMQEventEvents> for ::windows::runtime::IUnknown {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_DMSMQEventEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _DMSMQEventEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_DMSMQEventEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_DMSMQEventEvents> for super::Ole::Automation::IDispatch {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_DMSMQEventEvents> for super::Ole::Automation::IDispatch {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for _DMSMQEventEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &_DMSMQEventEvents
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _DMSMQEventEvents_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
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
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
