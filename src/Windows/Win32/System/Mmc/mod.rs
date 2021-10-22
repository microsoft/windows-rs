#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const AUTO_WIDTH: i32 = -1i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AppEvents(::windows::runtime::IUnknown);
impl AppEvents {
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
}
unsafe impl ::windows::runtime::Interface for AppEvents {
    type Vtable = AppEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4235870802,
        30892,
        17714,
        [140, 90, 86, 60, 254, 19, 136, 99],
    );
}
impl ::std::convert::From<AppEvents> for ::windows::runtime::IUnknown {
    fn from(value: AppEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppEvents> for ::windows::runtime::IUnknown {
    fn from(value: &AppEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<AppEvents> for super::OleAutomation::IDispatch {
    fn from(value: AppEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&AppEvents> for super::OleAutomation::IDispatch {
    fn from(value: &AppEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &AppEvents {
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
pub struct AppEvents_abi(
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
);
pub const AppEventsDHTMLConnector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2917549131,
    51487,
    20023,
    [146, 164, 91, 180, 48, 163, 51, 64],
);
pub const Application: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1236433178,
    45486,
    19600,
    [155, 142, 232, 96, 186, 7, 248, 137],
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
pub struct CCM_COMMANDID_MASK_CONSTANTS(pub u32);
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS =
    CCM_COMMANDID_MASK_CONSTANTS(4294901760u32);
impl ::std::convert::From<u32> for CCM_COMMANDID_MASK_CONSTANTS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CCM_COMMANDID_MASK_CONSTANTS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CCM_COMMANDID_MASK_CONSTANTS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CCM_COMMANDID_MASK_CONSTANTS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CCM_COMMANDID_MASK_CONSTANTS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CCM_COMMANDID_MASK_CONSTANTS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CCM_COMMANDID_MASK_CONSTANTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct CCM_INSERTIONALLOWED(pub i32);
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(1i32);
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(2i32);
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(4i32);
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(8i32);
impl ::std::convert::From<i32> for CCM_INSERTIONALLOWED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CCM_INSERTIONALLOWED {
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
pub struct CCM_INSERTIONPOINTID(pub i32);
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-65536i32);
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-2147483648i32);
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(1073741824i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(536870912i32);
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(268435456i32);
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(268369920i32);
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(31i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1610612736i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1610612735i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1610612734i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1610612733i32);
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1610612732i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1879048191i32);
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-1879048190i32);
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID =
    CCM_INSERTIONPOINTID(-2147483648i32);
impl ::std::convert::From<i32> for CCM_INSERTIONPOINTID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CCM_INSERTIONPOINTID {
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
pub struct CCM_SPECIAL(pub i32);
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = CCM_SPECIAL(1i32);
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = CCM_SPECIAL(2i32);
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = CCM_SPECIAL(4i32);
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = CCM_SPECIAL(8i32);
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = CCM_SPECIAL(16i32);
impl ::std::convert::From<i32> for CCM_SPECIAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CCM_SPECIAL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl CONTEXTMENUITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONTEXTMENUITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONTEXTMENUITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONTEXTMENUITEM")
            .field("strName", &self.strName)
            .field("strStatusBarText", &self.strStatusBarText)
            .field("lCommandID", &self.lCommandID)
            .field("lInsertionPointID", &self.lInsertionPointID)
            .field("fFlags", &self.fFlags)
            .field("fSpecialFlags", &self.fSpecialFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONTEXTMENUITEM {
    fn eq(&self, other: &Self) -> bool {
        self.strName == other.strName
            && self.strStatusBarText == other.strStatusBarText
            && self.lCommandID == other.lCommandID
            && self.lInsertionPointID == other.lInsertionPointID
            && self.fFlags == other.fFlags
            && self.fSpecialFlags == other.fSpecialFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONTEXTMENUITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONTEXTMENUITEM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONTEXTMENUITEM2 {
    pub strName: super::super::Foundation::PWSTR,
    pub strStatusBarText: super::super::Foundation::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CONTEXTMENUITEM2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONTEXTMENUITEM2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONTEXTMENUITEM2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONTEXTMENUITEM2")
            .field("strName", &self.strName)
            .field("strStatusBarText", &self.strStatusBarText)
            .field("lCommandID", &self.lCommandID)
            .field("lInsertionPointID", &self.lInsertionPointID)
            .field("fFlags", &self.fFlags)
            .field("fSpecialFlags", &self.fSpecialFlags)
            .field(
                "strLanguageIndependentName",
                &self.strLanguageIndependentName,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONTEXTMENUITEM2 {
    fn eq(&self, other: &Self) -> bool {
        self.strName == other.strName
            && self.strStatusBarText == other.strStatusBarText
            && self.lCommandID == other.lCommandID
            && self.lInsertionPointID == other.lInsertionPointID
            && self.fFlags == other.fFlags
            && self.fSpecialFlags == other.fSpecialFlags
            && self.strLanguageIndependentName == other.strLanguageIndependentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONTEXTMENUITEM2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONTEXTMENUITEM2 {
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
pub struct Column(::windows::runtime::IUnknown);
impl Column {
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
        )
        .ok()
    }
    pub unsafe fn DisplayPosition(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetDisplayPosition(&self, index: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hidden(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHidden<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hidden: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            hidden.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAsSortColumn(
        &self,
        sortorder: _ColumnSortOrder,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sortorder),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSortColumn(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Column {
    type Vtable = Column_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4246495075,
        11030,
        19718,
        [154, 179, 244, 83, 80, 185, 64, 171],
    );
}
impl ::std::convert::From<Column> for ::windows::runtime::IUnknown {
    fn from(value: Column) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Column> for ::windows::runtime::IUnknown {
    fn from(value: &Column) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Column {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Column {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Column> for super::OleAutomation::IDispatch {
    fn from(value: Column) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Column> for super::OleAutomation::IDispatch {
    fn from(value: &Column) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Column {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Column {
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
pub struct Column_abi(
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
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        displayposition: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hidden: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hidden: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sortorder: _ColumnSortOrder,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        issortcolumn: *mut super::super::Foundation::BOOL,
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
pub struct Columns(::windows::runtime::IUnknown);
impl Columns {
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
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<Column> {
        let mut result__: <Column as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<Column>(result__)
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
unsafe impl ::windows::runtime::Interface for Columns {
    type Vtable = Columns_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        943541655,
        64580,
        18315,
        [177, 57, 99, 35, 220, 72, 97, 28],
    );
}
impl ::std::convert::From<Columns> for ::windows::runtime::IUnknown {
    fn from(value: Columns) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Columns> for ::windows::runtime::IUnknown {
    fn from(value: &Columns) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Columns {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Columns {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Columns> for super::OleAutomation::IDispatch {
    fn from(value: Columns) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Columns> for super::OleAutomation::IDispatch {
    fn from(value: &Columns) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Columns {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Columns {
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
pub struct Columns_abi(
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
        index: i32,
        column: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
pub const ConsolePower: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4029174644,
    57329,
    4563,
    [180, 51, 0, 192, 79, 142, 205, 120],
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ContextMenu(::windows::runtime::IUnknown);
impl ContextMenu {
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
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Item<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        indexorpath: Param0,
    ) -> ::windows::runtime::Result<MenuItem> {
        let mut result__: <MenuItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            indexorpath.into_param().abi(),
            &mut result__,
        )
        .from_abi::<MenuItem>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ContextMenu {
    type Vtable = ContextMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3669204192,
        9702,
        19975,
        [131, 98, 186, 156, 149, 112, 101, 69],
    );
}
impl ::std::convert::From<ContextMenu> for ::windows::runtime::IUnknown {
    fn from(value: ContextMenu) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContextMenu> for ::windows::runtime::IUnknown {
    fn from(value: &ContextMenu) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContextMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContextMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ContextMenu> for super::OleAutomation::IDispatch {
    fn from(value: ContextMenu) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ContextMenu> for super::OleAutomation::IDispatch {
    fn from(value: &ContextMenu) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for ContextMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &ContextMenu {
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
pub struct ContextMenu_abi(
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
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        indexorpath: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        menuitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
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
pub struct DATA_OBJECT_TYPES(pub i32);
pub const CCT_SCOPE: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32768i32);
pub const CCT_RESULT: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32769i32);
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32770i32);
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(65535i32);
impl ::std::convert::From<i32> for DATA_OBJECT_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DATA_OBJECT_TYPES {
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
pub struct Document(::windows::runtime::IUnknown);
impl Document {
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
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        filename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            filename.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        savechanges: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            savechanges.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Views(&self) -> ::windows::runtime::Result<Views> {
        let mut result__: <Views as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Views>(result__)
    }
    pub unsafe fn SnapIns(&self) -> ::windows::runtime::Result<SnapIns> {
        let mut result__: <SnapIns as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<SnapIns>(result__)
    }
    pub unsafe fn ActiveView(&self) -> ::windows::runtime::Result<View> {
        let mut result__: <View as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<View>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            name.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Location(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSaved(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Mode(&self) -> ::windows::runtime::Result<_DocumentMode> {
        let mut result__: <_DocumentMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<_DocumentMode>(result__)
    }
    pub unsafe fn SetMode(&self, mode: _DocumentMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    pub unsafe fn RootNode(&self) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn ScopeNamespace(&self) -> ::windows::runtime::Result<ScopeNamespace> {
        let mut result__: <ScopeNamespace as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ScopeNamespace>(result__)
    }
    pub unsafe fn CreateProperties(&self) -> ::windows::runtime::Result<Properties> {
        let mut result__: <Properties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Properties>(result__)
    }
    pub unsafe fn Application(&self) -> ::windows::runtime::Result<_Application> {
        let mut result__: <_Application as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<_Application>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Document {
    type Vtable = Document_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        575742166,
        7695,
        16547,
        [147, 254, 16, 121, 230, 168, 1, 123],
    );
}
impl ::std::convert::From<Document> for ::windows::runtime::IUnknown {
    fn from(value: Document) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Document> for ::windows::runtime::IUnknown {
    fn from(value: &Document) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Document {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Document {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Document> for super::OleAutomation::IDispatch {
    fn from(value: Document) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Document> for super::OleAutomation::IDispatch {
    fn from(value: &Document) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Document {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Document {
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
pub struct Document_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        savechanges: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        views: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapins: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        location: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        issaved: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: *mut _DocumentMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: _DocumentMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenamespace: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        properties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        application: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Extension(::windows::runtime::IUnknown);
impl Extension {
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
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Vendor(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Extensions(&self) -> ::windows::runtime::Result<Extensions> {
        let mut result__: <Extensions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Extensions>(result__)
    }
    pub unsafe fn SnapinCLSID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        enable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            enable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        enable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            enable.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for Extension {
    type Vtable = Extension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2907532454,
        37167,
        16539,
        [162, 110, 127, 210, 52, 174, 245, 66],
    );
}
impl ::std::convert::From<Extension> for ::windows::runtime::IUnknown {
    fn from(value: Extension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Extension> for ::windows::runtime::IUnknown {
    fn from(value: &Extension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Extension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Extension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Extension> for super::OleAutomation::IDispatch {
    fn from(value: Extension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Extension> for super::OleAutomation::IDispatch {
    fn from(value: &Extension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Extension {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Extension {
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
pub struct Extension_abi(
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
        name: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vendor: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        version: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        extensions: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapinclsid: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: super::super::Foundation::BOOL,
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
pub struct Extensions(::windows::runtime::IUnknown);
impl Extensions {
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
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<Extension> {
        let mut result__: <Extension as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<Extension>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Extensions {
    type Vtable = Extensions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2195450435,
        36004,
        17596,
        [162, 202, 209, 135, 65, 5, 158, 200],
    );
}
impl ::std::convert::From<Extensions> for ::windows::runtime::IUnknown {
    fn from(value: Extensions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Extensions> for ::windows::runtime::IUnknown {
    fn from(value: &Extensions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Extensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Extensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Extensions> for super::OleAutomation::IDispatch {
    fn from(value: Extensions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Extensions> for super::OleAutomation::IDispatch {
    fn from(value: &Extensions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Extensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Extensions {
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
pub struct Extensions_abi(
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
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        extension: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Frame(::windows::runtime::IUnknown);
impl Frame {
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
    pub unsafe fn Maximize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Minimize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(top),
        )
        .ok()
    }
    pub unsafe fn Bottom(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetBottom(&self, bottom: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bottom),
        )
        .ok()
    }
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(left),
        )
        .ok()
    }
    pub unsafe fn Right(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetRight(&self, right: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(right),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for Frame {
    type Vtable = Frame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3856849264,
        23475,
        17158,
        [136, 4, 176, 150, 138, 49, 200, 230],
    );
}
impl ::std::convert::From<Frame> for ::windows::runtime::IUnknown {
    fn from(value: Frame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Frame> for ::windows::runtime::IUnknown {
    fn from(value: &Frame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Frame> for super::OleAutomation::IDispatch {
    fn from(value: Frame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Frame> for super::OleAutomation::IDispatch {
    fn from(value: &Frame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Frame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Frame {
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
pub struct Frame_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: i32,
    ) -> ::windows::runtime::HRESULT,
);
pub const HDI_HIDDEN: u32 = 1u32;
pub const HIDE_COLUMN: i32 = -4i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IColumnData(::windows::runtime::IUnknown);
impl IColumnData {
    pub unsafe fn SetColumnConfigData(
        &self,
        pcolid: *const SColumnSetID,
        pcolsetdata: *const MMC_COLUMN_SET_DATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolid),
            ::std::mem::transmute(pcolsetdata),
        )
        .ok()
    }
    pub unsafe fn GetColumnConfigData(
        &self,
        pcolid: *const SColumnSetID,
    ) -> ::windows::runtime::Result<*mut MMC_COLUMN_SET_DATA> {
        let mut result__: <*mut MMC_COLUMN_SET_DATA as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolid),
            &mut result__,
        )
        .from_abi::<*mut MMC_COLUMN_SET_DATA>(result__)
    }
    pub unsafe fn SetColumnSortData(
        &self,
        pcolid: *const SColumnSetID,
        pcolsortdata: *const MMC_SORT_SET_DATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolid),
            ::std::mem::transmute(pcolsortdata),
        )
        .ok()
    }
    pub unsafe fn GetColumnSortData(
        &self,
        pcolid: *const SColumnSetID,
    ) -> ::windows::runtime::Result<*mut MMC_SORT_SET_DATA> {
        let mut result__: <*mut MMC_SORT_SET_DATA as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcolid),
            &mut result__,
        )
        .from_abi::<*mut MMC_SORT_SET_DATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IColumnData {
    type Vtable = IColumnData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1417417556,
        589,
        4563,
        [167, 7, 0, 192, 79, 142, 244, 203],
    );
}
impl ::std::convert::From<IColumnData> for ::windows::runtime::IUnknown {
    fn from(value: IColumnData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IColumnData> for ::windows::runtime::IUnknown {
    fn from(value: &IColumnData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IColumnData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IColumnData {
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
pub struct IColumnData_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolid: *const SColumnSetID,
        pcolsetdata: *const MMC_COLUMN_SET_DATA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolid: *const SColumnSetID,
        ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolid: *const SColumnSetID,
        pcolsortdata: *const MMC_SORT_SET_DATA,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcolid: *const SColumnSetID,
        ppcolsortdata: *mut *mut MMC_SORT_SET_DATA,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IComponent(::windows::runtime::IUnknown);
impl IComponent {
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IConsole>>(
        &self,
        lpconsole: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            lpconsole.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        event: MMC_NOTIFY_TYPE,
        arg: Param2,
        param3: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            ::std::mem::transmute(event),
            arg.into_param().abi(),
            param3.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::Com::IDataObject> {
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType(
        &self,
        cookie: isize,
        ppviewtype: *mut super::super::Foundation::PWSTR,
        pviewoptions: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(ppviewtype),
            ::std::mem::transmute(pviewoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(
        &self,
        presultdataitem: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(presultdataitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobjecta: Param0,
        lpdataobjectb: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpdataobjecta.into_param().abi(),
            lpdataobjectb.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComponent {
    type Vtable = IComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125346994,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IComponent> for ::windows::runtime::IUnknown {
    fn from(value: IComponent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponent> for ::windows::runtime::IUnknown {
    fn from(value: &IComponent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComponent {
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
pub struct IComponent_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpconsole: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        event: MMC_NOTIFY_TYPE,
        arg: super::super::Foundation::LPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdataobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        ppviewtype: *mut super::super::Foundation::PWSTR,
        pviewoptions: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        presultdataitem: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobjecta: ::windows::runtime::RawPtr,
        lpdataobjectb: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IComponent2(::windows::runtime::IUnknown);
impl IComponent2 {
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IConsole>>(
        &self,
        lpconsole: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            lpconsole.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        event: MMC_NOTIFY_TYPE,
        arg: Param2,
        param3: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            ::std::mem::transmute(event),
            arg.into_param().abi(),
            param3.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::Com::IDataObject> {
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType(
        &self,
        cookie: isize,
        ppviewtype: *mut super::super::Foundation::PWSTR,
        pviewoptions: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(ppviewtype),
            ::std::mem::transmute(pviewoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(
        &self,
        presultdataitem: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(presultdataitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobjecta: Param0,
        lpdataobjectb: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpdataobjecta.into_param().abi(),
            lpdataobjectb.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn QueryDispatch(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultViewType2(
        &self,
        cookie: isize,
        presultviewtype: *mut RESULT_VIEW_TYPE_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(presultviewtype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreResultView(
        &self,
        cookie: isize,
        presultviewtype: *const RESULT_VIEW_TYPE_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(presultviewtype),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComponent2 {
    type Vtable = IComponent2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2040714773,
        18960,
        20180,
        [140, 101, 134, 51, 249, 51, 80, 149],
    );
}
impl ::std::convert::From<IComponent2> for ::windows::runtime::IUnknown {
    fn from(value: IComponent2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponent2> for ::windows::runtime::IUnknown {
    fn from(value: &IComponent2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComponent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IComponent2> for IComponent {
    fn from(value: IComponent2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponent2> for IComponent {
    fn from(value: &IComponent2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComponent> for IComponent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComponent> for &IComponent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpconsole: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        event: MMC_NOTIFY_TYPE,
        arg: super::super::Foundation::LPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdataobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        ppviewtype: *mut super::super::Foundation::PWSTR,
        pviewoptions: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        presultdataitem: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobjecta: ::windows::runtime::RawPtr,
        lpdataobjectb: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdispatch: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        presultviewtype: *mut ::std::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        presultviewtype: *const ::std::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO>,
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
pub struct IComponentData(::windows::runtime::IUnknown);
impl IComponentData {
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        punknown: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateComponent(&self) -> ::windows::runtime::Result<IComponent> {
        let mut result__: <IComponent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IComponent>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        event: MMC_NOTIFY_TYPE,
        arg: Param2,
        param3: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            ::std::mem::transmute(event),
            arg.into_param().abi(),
            param3.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::Com::IDataObject> {
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(
        &self,
        pscopedataitem: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pscopedataitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobjecta: Param0,
        lpdataobjectb: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpdataobjecta.into_param().abi(),
            lpdataobjectb.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComponentData {
    type Vtable = IComponentData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2505749130,
        21016,
        4560,
        [169, 133, 0, 192, 79, 216, 213, 101],
    );
}
impl ::std::convert::From<IComponentData> for ::windows::runtime::IUnknown {
    fn from(value: IComponentData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponentData> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComponentData {
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
pub struct IComponentData_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcomponent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        event: MMC_NOTIFY_TYPE,
        arg: super::super::Foundation::LPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdataobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscopedataitem: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobjecta: ::windows::runtime::RawPtr,
        lpdataobjectb: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IComponentData2(::windows::runtime::IUnknown);
impl IComponentData2 {
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        punknown: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            punknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CreateComponent(&self) -> ::windows::runtime::Result<IComponent> {
        let mut result__: <IComponent as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IComponent>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        event: MMC_NOTIFY_TYPE,
        arg: Param2,
        param3: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            ::std::mem::transmute(event),
            arg.into_param().abi(),
            param3.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::Com::IDataObject> {
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(
        &self,
        pscopedataitem: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pscopedataitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobjecta: Param0,
        lpdataobjectb: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpdataobjecta.into_param().abi(),
            lpdataobjectb.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn QueryDispatch(
        &self,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cookie),
            ::std::mem::transmute(r#type),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IComponentData2 {
    type Vtable = IComponentData2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3433099986,
        33502,
        16821,
        [191, 71, 59, 32, 118, 39, 61, 92],
    );
}
impl ::std::convert::From<IComponentData2> for ::windows::runtime::IUnknown {
    fn from(value: IComponentData2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponentData2> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentData2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComponentData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IComponentData2> for IComponentData {
    fn from(value: IComponentData2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComponentData2> for IComponentData {
    fn from(value: &IComponentData2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComponentData> for IComponentData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComponentData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IComponentData>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IComponentData> for &IComponentData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IComponentData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IComponentData>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppcomponent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        event: MMC_NOTIFY_TYPE,
        arg: super::super::Foundation::LPARAM,
        param3: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdataobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pscopedataitem: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobjecta: ::windows::runtime::RawPtr,
        lpdataobjectb: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: isize,
        r#type: DATA_OBJECT_TYPES,
        ppdispatch: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsole(::windows::runtime::IUnknown);
impl IConsole {
    pub unsafe fn SetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, IHeaderCtrl>>(
        &self,
        pheader: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pheader.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetToolbar<'a, Param0: ::windows::runtime::IntoParam<'a, IToolbar>>(
        &self,
        ptoolbar: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ptoolbar.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryResultView(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        data: Param1,
        hint: isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            data.into_param().abi(),
            ::std::mem::transmute(hint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        lpsztext: Param0,
        lpsztitle: Param1,
        fustyle: u32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpsztext.into_param().abi(),
            lpsztitle.into_param().abi(),
            ::std::mem::transmute(fustyle),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::runtime::Result<IConsoleVerb> {
        let mut result__: <IConsoleVerb as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IConsoleVerb>(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn NewWindow(
        &self,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
            ::std::mem::transmute(loptions),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsole {
    type Vtable = IConsole_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125346993,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IConsole> for ::windows::runtime::IUnknown {
    fn from(value: IConsole) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole> for ::windows::runtime::IUnknown {
    fn from(value: &IConsole) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsole {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsole {
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
pub struct IConsole_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pheader: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptoolbar: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punknown: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        data: super::super::Foundation::LPARAM,
        hint: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpsztext: super::super::Foundation::PWSTR,
        lpsztitle: super::super::Foundation::PWSTR,
        fustyle: u32,
        piretval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppconsoleverb: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsole2(::windows::runtime::IUnknown);
impl IConsole2 {
    pub unsafe fn SetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, IHeaderCtrl>>(
        &self,
        pheader: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pheader.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetToolbar<'a, Param0: ::windows::runtime::IntoParam<'a, IToolbar>>(
        &self,
        ptoolbar: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ptoolbar.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryResultView(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        data: Param1,
        hint: isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            data.into_param().abi(),
            ::std::mem::transmute(hint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        lpsztext: Param0,
        lpsztitle: Param1,
        fustyle: u32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpsztext.into_param().abi(),
            lpsztitle.into_param().abi(),
            ::std::mem::transmute(fustyle),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::runtime::Result<IConsoleVerb> {
        let mut result__: <IConsoleVerb as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IConsoleVerb>(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn NewWindow(
        &self,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
            ::std::mem::transmute(loptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hitem: isize,
        bexpand: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            bexpand.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszstatustext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pszstatustext.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsole2 {
    type Vtable = IConsole2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        272466986,
        43619,
        4561,
        [167, 225, 0, 192, 79, 216, 213, 101],
    );
}
impl ::std::convert::From<IConsole2> for ::windows::runtime::IUnknown {
    fn from(value: IConsole2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole2> for ::windows::runtime::IUnknown {
    fn from(value: &IConsole2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsole2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsole2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IConsole2> for IConsole {
    fn from(value: IConsole2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole2> for IConsole {
    fn from(value: &IConsole2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole> for IConsole2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole> for &IConsole2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pheader: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptoolbar: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punknown: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        data: super::super::Foundation::LPARAM,
        hint: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpsztext: super::super::Foundation::PWSTR,
        lpsztitle: super::super::Foundation::PWSTR,
        fustyle: u32,
        piretval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppconsoleverb: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        bexpand: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszstatustext: super::super::Foundation::PWSTR,
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
pub struct IConsole3(::windows::runtime::IUnknown);
impl IConsole3 {
    pub unsafe fn SetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, IHeaderCtrl>>(
        &self,
        pheader: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pheader.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetToolbar<'a, Param0: ::windows::runtime::IntoParam<'a, IToolbar>>(
        &self,
        ptoolbar: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ptoolbar.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryResultView(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::runtime::Result<IImageList> {
        let mut result__: <IImageList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IImageList>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lpdataobject: Param0,
        data: Param1,
        hint: isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
            data.into_param().abi(),
            ::std::mem::transmute(hint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBox<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        lpsztext: Param0,
        lpsztitle: Param1,
        fustyle: u32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            lpsztext.into_param().abi(),
            lpsztitle.into_param().abi(),
            ::std::mem::transmute(fustyle),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::runtime::Result<IConsoleVerb> {
        let mut result__: <IConsoleVerb as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IConsoleVerb>(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn NewWindow(
        &self,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
            ::std::mem::transmute(loptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hitem: isize,
        bexpand: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            bexpand.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszstatustext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            pszstatustext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RenameScopeItem(&self, hscopeitem: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hscopeitem),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsole3 {
    type Vtable = IConsole3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1334177755,
        53473,
        18828,
        [141, 74, 208, 16, 223, 221, 64, 79],
    );
}
impl ::std::convert::From<IConsole3> for ::windows::runtime::IUnknown {
    fn from(value: IConsole3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole3> for ::windows::runtime::IUnknown {
    fn from(value: &IConsole3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IConsole3> for IConsole2 {
    fn from(value: IConsole3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole3> for IConsole2 {
    fn from(value: &IConsole3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole2> for IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole2> for &IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IConsole3> for IConsole {
    fn from(value: IConsole3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsole3> for IConsole {
    fn from(value: &IConsole3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole> for IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsole> for &IConsole3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsole> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsole>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pheader: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptoolbar: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punknown: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppimagelist: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
        data: super::super::Foundation::LPARAM,
        hint: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpsztext: super::super::Foundation::PWSTR,
        lpsztitle: super::super::Foundation::PWSTR,
        fustyle: u32,
        piretval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppconsoleverb: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
        loptions: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        bexpand: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszstatustext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hscopeitem: isize,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsoleNameSpace(::windows::runtime::IUnknown);
impl IConsoleNameSpace {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn DeleteItem(
        &self,
        hitem: isize,
        fdeletethis: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            ::std::mem::transmute(fdeletethis),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn GetChildItem(
        &self,
        item: isize,
        pitemchild: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemchild),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
    pub unsafe fn GetNextItem(
        &self,
        item: isize,
        pitemnext: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemnext),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
    pub unsafe fn GetParentItem(
        &self,
        item: isize,
        pitemparent: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemparent),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsoleNameSpace {
    type Vtable = IConsoleNameSpace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3202266656,
        62029,
        4559,
        [138, 252, 0, 170, 0, 60, 169, 246],
    );
}
impl ::std::convert::From<IConsoleNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: IConsoleNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsoleNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: &IConsoleNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsoleNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsoleNameSpace {
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
pub struct IConsoleNameSpace_abi(
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
        item: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        fdeletethis: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *const SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemchild: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemnext: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemparent: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsoleNameSpace2(::windows::runtime::IUnknown);
impl IConsoleNameSpace2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn DeleteItem(
        &self,
        hitem: isize,
        fdeletethis: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            ::std::mem::transmute(fdeletethis),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn GetChildItem(
        &self,
        item: isize,
        pitemchild: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemchild),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
    pub unsafe fn GetNextItem(
        &self,
        item: isize,
        pitemnext: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemnext),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
    pub unsafe fn GetParentItem(
        &self,
        item: isize,
        pitemparent: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(pitemparent),
            ::std::mem::transmute(pcookie),
        )
        .ok()
    }
    pub unsafe fn Expand(&self, hitem: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
        )
        .ok()
    }
    pub unsafe fn AddExtension(
        &self,
        hitem: isize,
        lpclsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            ::std::mem::transmute(lpclsid),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsoleNameSpace2 {
    type Vtable = IConsoleNameSpace2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        626989260,
        26075,
        4561,
        [167, 220, 0, 192, 79, 216, 213, 101],
    );
}
impl ::std::convert::From<IConsoleNameSpace2> for ::windows::runtime::IUnknown {
    fn from(value: IConsoleNameSpace2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsoleNameSpace2> for ::windows::runtime::IUnknown {
    fn from(value: &IConsoleNameSpace2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IConsoleNameSpace2> for IConsoleNameSpace {
    fn from(value: IConsoleNameSpace2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsoleNameSpace2> for IConsoleNameSpace {
    fn from(value: &IConsoleNameSpace2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsoleNameSpace> for IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsoleNameSpace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsoleNameSpace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IConsoleNameSpace> for &IConsoleNameSpace2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IConsoleNameSpace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IConsoleNameSpace>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace2_abi(
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
        item: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        fdeletethis: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *const SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut SCOPEDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemchild: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemnext: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: isize,
        pitemparent: *mut isize,
        pcookie: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        lpclsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsolePower(::windows::runtime::IUnknown);
impl IConsolePower {
    pub unsafe fn SetExecutionState(
        &self,
        dwadd: u32,
        dwremove: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwadd),
            ::std::mem::transmute(dwremove),
        )
        .ok()
    }
    pub unsafe fn ResetIdleTimer(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConsolePower {
    type Vtable = IConsolePower_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        486268174,
        25290,
        18894,
        [163, 175, 219, 178, 222, 97, 176, 104],
    );
}
impl ::std::convert::From<IConsolePower> for ::windows::runtime::IUnknown {
    fn from(value: IConsolePower) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsolePower> for ::windows::runtime::IUnknown {
    fn from(value: &IConsolePower) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsolePower {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsolePower {
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
pub struct IConsolePower_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwadd: u32,
        dwremove: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IConsolePowerSink(::windows::runtime::IUnknown);
impl IConsolePowerSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPowerBroadcast<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        nevent: u32,
        lparam: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::LRESULT> {
        let mut result__: <super::super::Foundation::LRESULT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nevent),
            lparam.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::LRESULT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConsolePowerSink {
    type Vtable = IConsolePowerSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        859010463,
        65103,
        18805,
        [177, 67, 254, 192, 165, 221, 109, 101],
    );
}
impl ::std::convert::From<IConsolePowerSink> for ::windows::runtime::IUnknown {
    fn from(value: IConsolePowerSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsolePowerSink> for ::windows::runtime::IUnknown {
    fn from(value: &IConsolePowerSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsolePowerSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsolePowerSink {
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
pub struct IConsolePowerSink_abi(
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
        nevent: u32,
        lparam: super::super::Foundation::LPARAM,
        plreturn: *mut super::super::Foundation::LRESULT,
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
pub struct IConsoleVerb(::windows::runtime::IUnknown);
impl IConsoleVerb {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVerbState(
        &self,
        ecmdid: MMC_CONSOLE_VERB,
        nstate: MMC_BUTTON_STATE,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ecmdid),
            ::std::mem::transmute(nstate),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVerbState<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        ecmdid: MMC_CONSOLE_VERB,
        nstate: MMC_BUTTON_STATE,
        bstate: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ecmdid),
            ::std::mem::transmute(nstate),
            bstate.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetDefaultVerb(
        &self,
        ecmdid: MMC_CONSOLE_VERB,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ecmdid),
        )
        .ok()
    }
    pub unsafe fn GetDefaultVerb(&self) -> ::windows::runtime::Result<MMC_CONSOLE_VERB> {
        let mut result__: <MMC_CONSOLE_VERB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<MMC_CONSOLE_VERB>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConsoleVerb {
    type Vtable = IConsoleVerb_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3835656800,
        29871,
        4560,
        [162, 134, 0, 192, 79, 216, 254, 147],
    );
}
impl ::std::convert::From<IConsoleVerb> for ::windows::runtime::IUnknown {
    fn from(value: IConsoleVerb) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConsoleVerb> for ::windows::runtime::IUnknown {
    fn from(value: &IConsoleVerb) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConsoleVerb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConsoleVerb {
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
pub struct IConsoleVerb_abi(
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
        ecmdid: MMC_CONSOLE_VERB,
        nstate: MMC_BUTTON_STATE,
        pstate: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ecmdid: MMC_CONSOLE_VERB,
        nstate: MMC_BUTTON_STATE,
        bstate: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ecmdid: MMC_CONSOLE_VERB,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pecmdid: *mut MMC_CONSOLE_VERB,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IContextMenuCallback(::windows::runtime::IUnknown);
impl IContextMenuCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pitem),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextMenuCallback {
    type Vtable = IContextMenuCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125346999,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IContextMenuCallback> for ::windows::runtime::IUnknown {
    fn from(value: IContextMenuCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContextMenuCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IContextMenuCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextMenuCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContextMenuCallback {
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
pub struct IContextMenuCallback_abi(
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
        pitem: *const CONTEXTMENUITEM,
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
pub struct IContextMenuCallback2(::windows::runtime::IUnknown);
impl IContextMenuCallback2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pitem),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextMenuCallback2 {
    type Vtable = IContextMenuCallback2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3782786062,
        11984,
        19294,
        [128, 151, 66, 201, 8, 126, 139, 51],
    );
}
impl ::std::convert::From<IContextMenuCallback2> for ::windows::runtime::IUnknown {
    fn from(value: IContextMenuCallback2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContextMenuCallback2> for ::windows::runtime::IUnknown {
    fn from(value: &IContextMenuCallback2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextMenuCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IContextMenuCallback2
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
pub struct IContextMenuCallback2_abi(
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
        pitem: *const CONTEXTMENUITEM2,
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
pub struct IContextMenuProvider(::windows::runtime::IUnknown);
impl IContextMenuProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pitem),
        )
        .ok()
    }
    pub unsafe fn EmptyMenuList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPrimaryExtensionItems<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        piextension: Param0,
        pidataobject: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            piextension.into_param().abi(),
            pidataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddThirdPartyExtensionItems<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        pidataobject: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pidataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowContextMenu<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        xpos: i32,
        ypos: i32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(xpos),
            ::std::mem::transmute(ypos),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContextMenuProvider {
    type Vtable = IContextMenuProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125346998,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IContextMenuProvider> for ::windows::runtime::IUnknown {
    fn from(value: IContextMenuProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContextMenuProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IContextMenuProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextMenuProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContextMenuProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IContextMenuProvider> for IContextMenuCallback {
    fn from(value: IContextMenuProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContextMenuProvider> for IContextMenuCallback {
    fn from(value: &IContextMenuProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContextMenuCallback> for IContextMenuProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContextMenuCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContextMenuCallback>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContextMenuCallback> for &IContextMenuProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContextMenuCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContextMenuCallback>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuProvider_abi(
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
        pitem: *const CONTEXTMENUITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piextension: ::windows::runtime::RawPtr,
        pidataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pidataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        xpos: i32,
        ypos: i32,
        plselected: *mut i32,
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
pub struct IControlbar(::windows::runtime::IUnknown);
impl IControlbar {
    pub unsafe fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, IExtendControlbar>>(
        &self,
        ntype: MMC_CONTROL_TYPE,
        pextendcontrolbar: Param1,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ntype),
            pextendcontrolbar.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Attach<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        ntype: MMC_CONTROL_TYPE,
        lpunknown: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ntype),
            lpunknown.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Detach<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        lpunknown: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lpunknown.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IControlbar {
    type Vtable = IControlbar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1778090270,
        27676,
        4560,
        [162, 203, 0, 192, 79, 217, 9, 221],
    );
}
impl ::std::convert::From<IControlbar> for ::windows::runtime::IUnknown {
    fn from(value: IControlbar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IControlbar> for ::windows::runtime::IUnknown {
    fn from(value: &IControlbar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IControlbar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IControlbar {
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
pub struct IControlbar_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ntype: MMC_CONTROL_TYPE,
        pextendcontrolbar: ::windows::runtime::RawPtr,
        ppunknown: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ntype: MMC_CONTROL_TYPE,
        lpunknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpunknown: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDisplayHelp(::windows::runtime::IUnknown);
impl IDisplayHelp {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowTopic<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszhelptopic: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszhelptopic.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDisplayHelp {
    type Vtable = IDisplayHelp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3428399152,
        47398,
        4561,
        [128, 99, 0, 0, 248, 117, 169, 206],
    );
}
impl ::std::convert::From<IDisplayHelp> for ::windows::runtime::IUnknown {
    fn from(value: IDisplayHelp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDisplayHelp> for ::windows::runtime::IUnknown {
    fn from(value: &IDisplayHelp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDisplayHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDisplayHelp {
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
pub struct IDisplayHelp_abi(
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
        pszhelptopic: super::super::Foundation::PWSTR,
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
pub struct IEnumTASK(::windows::runtime::IUnknown);
impl IEnumTASK {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut MMC_TASK,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTASK> {
        let mut result__: <IEnumTASK as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumTASK>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTASK {
    type Vtable = IEnumTASK_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        864458929,
        23042,
        4561,
        [159, 236, 0, 96, 8, 50, 219, 74],
    );
}
impl ::std::convert::From<IEnumTASK> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTASK) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumTASK> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTASK) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTASK {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumTASK {
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
pub struct IEnumTASK_abi(
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
        celt: u32,
        rgelt: *mut MMC_TASK,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IExtendContextMenu(::windows::runtime::IUnknown);
impl IExtendContextMenu {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddMenuItems<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, IContextMenuCallback>,
    >(
        &self,
        pidataobject: Param0,
        picallback: Param1,
        pinsertionallowed: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pidataobject.into_param().abi(),
            picallback.into_param().abi(),
            ::std::mem::transmute(pinsertionallowed),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Command<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lcommandid: i32,
        pidataobject: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lcommandid),
            pidataobject.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IExtendContextMenu {
    type Vtable = IExtendContextMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1329297999,
        53164,
        4559,
        [184, 227, 0, 192, 79, 216, 213, 176],
    );
}
impl ::std::convert::From<IExtendContextMenu> for ::windows::runtime::IUnknown {
    fn from(value: IExtendContextMenu) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendContextMenu> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendContextMenu) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendContextMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExtendContextMenu {
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
pub struct IExtendContextMenu_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pidataobject: ::windows::runtime::RawPtr,
        picallback: ::windows::runtime::RawPtr,
        pinsertionallowed: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lcommandid: i32,
        pidataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IExtendControlbar(::windows::runtime::IUnknown);
impl IExtendControlbar {
    pub unsafe fn SetControlbar<'a, Param0: ::windows::runtime::IntoParam<'a, IControlbar>>(
        &self,
        pcontrolbar: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pcontrolbar.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ControlbarNotify<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        event: MMC_NOTIFY_TYPE,
        arg: Param1,
        param2: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(event),
            arg.into_param().abi(),
            param2.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IExtendControlbar {
    type Vtable = IExtendControlbar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1230005536,
        28480,
        4560,
        [169, 139, 0, 192, 79, 216, 213, 101],
    );
}
impl ::std::convert::From<IExtendControlbar> for ::windows::runtime::IUnknown {
    fn from(value: IExtendControlbar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendControlbar> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendControlbar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendControlbar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExtendControlbar {
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
pub struct IExtendControlbar_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcontrolbar: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        event: MMC_NOTIFY_TYPE,
        arg: super::super::Foundation::LPARAM,
        param2: super::super::Foundation::LPARAM,
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
pub struct IExtendPropertySheet(::windows::runtime::IUnknown);
impl IExtendPropertySheet {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IPropertySheetCallback>,
        Param2: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpprovider: Param0,
        handle: isize,
        lpidataobject: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            lpprovider.into_param().abi(),
            ::std::mem::transmute(handle),
            lpidataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobject: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IExtendPropertySheet {
    type Vtable = IExtendPropertySheet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2245944540,
        61217,
        4559,
        [162, 133, 0, 192, 79, 216, 219, 230],
    );
}
impl ::std::convert::From<IExtendPropertySheet> for ::windows::runtime::IUnknown {
    fn from(value: IExtendPropertySheet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendPropertySheet> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendPropertySheet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendPropertySheet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExtendPropertySheet {
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
pub struct IExtendPropertySheet_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpprovider: ::windows::runtime::RawPtr,
        handle: isize,
        lpidataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IExtendPropertySheet2(::windows::runtime::IUnknown);
impl IExtendPropertySheet2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IPropertySheetCallback>,
        Param2: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpprovider: Param0,
        handle: isize,
        lpidataobject: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            lpprovider.into_param().abi(),
            ::std::mem::transmute(handle),
            lpidataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpdataobject: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            lpdataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com"
    ))]
    pub unsafe fn GetWatermarks<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        lpidataobject: Param0,
        lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP,
        lphheader: *mut super::super::Graphics::Gdi::HBITMAP,
        lphpalette: *mut super::super::Graphics::Gdi::HPALETTE,
        bstretch: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lpidataobject.into_param().abi(),
            ::std::mem::transmute(lphwatermark),
            ::std::mem::transmute(lphheader),
            ::std::mem::transmute(lphpalette),
            ::std::mem::transmute(bstretch),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IExtendPropertySheet2 {
    type Vtable = IExtendPropertySheet2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3081269810,
        19025,
        4561,
        [167, 234, 0, 192, 79, 217, 9, 221],
    );
}
impl ::std::convert::From<IExtendPropertySheet2> for ::windows::runtime::IUnknown {
    fn from(value: IExtendPropertySheet2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendPropertySheet2> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendPropertySheet2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IExtendPropertySheet2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IExtendPropertySheet2> for IExtendPropertySheet {
    fn from(value: IExtendPropertySheet2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendPropertySheet2> for IExtendPropertySheet {
    fn from(value: &IExtendPropertySheet2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IExtendPropertySheet> for IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IExtendPropertySheet> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IExtendPropertySheet>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IExtendPropertySheet> for &IExtendPropertySheet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IExtendPropertySheet> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IExtendPropertySheet>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpprovider: ::windows::runtime::RawPtr,
        handle: isize,
        lpidataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpidataobject: ::windows::runtime::RawPtr,
        lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP,
        lphheader: *mut super::super::Graphics::Gdi::HBITMAP,
        lphpalette: *mut super::super::Graphics::Gdi::HPALETTE,
        bstretch: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com"
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
pub struct IExtendTaskPad(::windows::runtime::IUnknown);
impl IExtendTaskPad {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn TaskNotify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        pdo: Param0,
        arg: *const super::OleAutomation::VARIANT,
        param2: *const super::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pdo.into_param().abi(),
            ::std::mem::transmute(arg),
            ::std::mem::transmute(param2),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumTasks<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pdo: Param0,
        sztaskgroup: Param1,
    ) -> ::windows::runtime::Result<IEnumTASK> {
        let mut result__: <IEnumTASK as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pdo.into_param().abi(),
            sztaskgroup.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IEnumTASK>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTitle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszgroup: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszgroup.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescriptiveText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszgroup: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pszgroup.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackground<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszgroup: Param0,
    ) -> ::windows::runtime::Result<MMC_TASK_DISPLAY_OBJECT> {
        let mut result__: <MMC_TASK_DISPLAY_OBJECT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pszgroup.into_param().abi(),
            &mut result__,
        )
        .from_abi::<MMC_TASK_DISPLAY_OBJECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetListPadInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszgroup: Param0,
    ) -> ::windows::runtime::Result<MMC_LISTPAD_INFO> {
        let mut result__: <MMC_LISTPAD_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            pszgroup.into_param().abi(),
            &mut result__,
        )
        .from_abi::<MMC_LISTPAD_INFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IExtendTaskPad {
    type Vtable = IExtendTaskPad_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2381210897,
        21837,
        4561,
        [159, 234, 0, 96, 8, 50, 219, 74],
    );
}
impl ::std::convert::From<IExtendTaskPad> for ::windows::runtime::IUnknown {
    fn from(value: IExtendTaskPad) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendTaskPad> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendTaskPad) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendTaskPad {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExtendTaskPad {
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
pub struct IExtendTaskPad_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdo: ::windows::runtime::RawPtr,
        arg: *const ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        param2: *const ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdo: ::windows::runtime::RawPtr,
        sztaskgroup: super::super::Foundation::PWSTR,
        ppenumtask: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszgroup: super::super::Foundation::PWSTR,
        psztitle: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszgroup: super::super::Foundation::PWSTR,
        pszdescriptivetext: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszgroup: super::super::Foundation::PWSTR,
        ptdo: *mut MMC_TASK_DISPLAY_OBJECT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszgroup: super::super::Foundation::PWSTR,
        lplistpadinfo: *mut MMC_LISTPAD_INFO,
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
pub struct IExtendView(::windows::runtime::IUnknown);
impl IExtendView {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetViews<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, IViewExtensionCallback>,
    >(
        &self,
        pdataobject: Param0,
        pviewextensioncallback: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pdataobject.into_param().abi(),
            pviewextensioncallback.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IExtendView {
    type Vtable = IExtendView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2308529390,
        53997,
        19470,
        [174, 94, 223, 126, 118, 243, 250, 83],
    );
}
impl ::std::convert::From<IExtendView> for ::windows::runtime::IUnknown {
    fn from(value: IExtendView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExtendView> for ::windows::runtime::IUnknown {
    fn from(value: &IExtendView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExtendView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExtendView {
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
pub struct IExtendView_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdataobject: ::windows::runtime::RawPtr,
        pviewextensioncallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IHeaderCtrl(::windows::runtime::IUnknown);
impl IHeaderCtrl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertColumn<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        ncol: i32,
        title: Param1,
        nformat: i32,
        nwidth: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            title.into_param().abi(),
            ::std::mem::transmute(nformat),
            ::std::mem::transmute(nwidth),
        )
        .ok()
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnText<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        ncol: i32,
        title: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            title.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnText(
        &self,
        ncol: i32,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            ::std::mem::transmute(nwidth),
        )
        .ok()
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IHeaderCtrl {
    type Vtable = IHeaderCtrl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125346995,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IHeaderCtrl> for ::windows::runtime::IUnknown {
    fn from(value: IHeaderCtrl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IHeaderCtrl> for ::windows::runtime::IUnknown {
    fn from(value: &IHeaderCtrl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHeaderCtrl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IHeaderCtrl {
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
pub struct IHeaderCtrl_abi(
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
        ncol: i32,
        title: super::super::Foundation::PWSTR,
        nformat: i32,
        nwidth: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        title: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        ptext: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        nwidth: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        pwidth: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IHeaderCtrl2(::windows::runtime::IUnknown);
impl IHeaderCtrl2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertColumn<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        ncol: i32,
        title: Param1,
        nformat: i32,
        nwidth: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            title.into_param().abi(),
            ::std::mem::transmute(nformat),
            ::std::mem::transmute(nwidth),
        )
        .ok()
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnText<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        ncol: i32,
        title: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            title.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnText(
        &self,
        ncol: i32,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            ::std::mem::transmute(nwidth),
        )
        .ok()
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncol),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetChangeTimeOut(&self, utimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(utimeout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumnFilter(
        &self,
        ncolumn: u32,
        dwtype: u32,
        pfilterdata: *const MMC_FILTERDATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncolumn),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(pfilterdata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumnFilter(
        &self,
        ncolumn: u32,
        pdwtype: *mut u32,
        pfilterdata: *mut MMC_FILTERDATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncolumn),
            ::std::mem::transmute(pdwtype),
            ::std::mem::transmute(pfilterdata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHeaderCtrl2 {
    type Vtable = IHeaderCtrl2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2539105208,
        6962,
        4561,
        [167, 206, 0, 192, 79, 216, 213, 101],
    );
}
impl ::std::convert::From<IHeaderCtrl2> for ::windows::runtime::IUnknown {
    fn from(value: IHeaderCtrl2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IHeaderCtrl2> for ::windows::runtime::IUnknown {
    fn from(value: &IHeaderCtrl2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHeaderCtrl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IHeaderCtrl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IHeaderCtrl2> for IHeaderCtrl {
    fn from(value: IHeaderCtrl2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IHeaderCtrl2> for IHeaderCtrl {
    fn from(value: &IHeaderCtrl2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IHeaderCtrl> for IHeaderCtrl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IHeaderCtrl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IHeaderCtrl>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IHeaderCtrl> for &IHeaderCtrl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IHeaderCtrl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IHeaderCtrl>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl2_abi(
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
        ncol: i32,
        title: super::super::Foundation::PWSTR,
        nformat: i32,
        nwidth: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        title: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        ptext: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        nwidth: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncol: i32,
        pwidth: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        utimeout: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncolumn: u32,
        dwtype: u32,
        pfilterdata: *const MMC_FILTERDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncolumn: u32,
        pdwtype: *mut u32,
        pfilterdata: *mut MMC_FILTERDATA,
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
pub struct IImageList(::windows::runtime::IUnknown);
impl IImageList {
    pub unsafe fn ImageListSetIcon(
        &self,
        picon: *const isize,
        nloc: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(picon),
            ::std::mem::transmute(nloc),
        )
        .ok()
    }
    pub unsafe fn ImageListSetStrip(
        &self,
        pbmapsm: *const isize,
        pbmaplg: *const isize,
        nstartloc: i32,
        cmask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbmapsm),
            ::std::mem::transmute(pbmaplg),
            ::std::mem::transmute(nstartloc),
            ::std::mem::transmute(cmask),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IImageList {
    type Vtable = IImageList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125347000,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IImageList> for ::windows::runtime::IUnknown {
    fn from(value: IImageList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IImageList> for ::windows::runtime::IUnknown {
    fn from(value: &IImageList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IImageList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IImageList {
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
pub struct IImageList_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        picon: *const isize,
        nloc: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbmapsm: *const isize,
        pbmaplg: *const isize,
        nstartloc: i32,
        cmask: u32,
    ) -> ::windows::runtime::HRESULT,
);
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMMCVersionInfo(::windows::runtime::IUnknown);
impl IMMCVersionInfo {
    pub unsafe fn GetMMCVersion(
        &self,
        pversionmajor: *mut i32,
        pversionminor: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pversionmajor),
            ::std::mem::transmute(pversionminor),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMMCVersionInfo {
    type Vtable = IMMCVersionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2832385534,
        52683,
        19357,
        [189, 229, 162, 115, 67, 255, 84, 188],
    );
}
impl ::std::convert::From<IMMCVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMMCVersionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMMCVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMMCVersionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMCVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMMCVersionInfo {
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
pub struct IMMCVersionInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pversionmajor: *mut i32,
        pversionminor: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMenuButton(::windows::runtime::IUnknown);
impl IMenuButton {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddButton<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        idcommand: i32,
        lpbuttontext: Param1,
        lptooltiptext: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idcommand),
            lpbuttontext.into_param().abi(),
            lptooltiptext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButton<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        idcommand: i32,
        lpbuttontext: Param1,
        lptooltiptext: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idcommand),
            lpbuttontext.into_param().abi(),
            lptooltiptext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
        bstate: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idcommand),
            ::std::mem::transmute(nstate),
            bstate.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMenuButton {
    type Vtable = IMenuButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2501826384,
        53376,
        4560,
        [177, 151, 0, 0, 0, 0, 0, 0],
    );
}
impl ::std::convert::From<IMenuButton> for ::windows::runtime::IUnknown {
    fn from(value: IMenuButton) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMenuButton> for ::windows::runtime::IUnknown {
    fn from(value: &IMenuButton) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMenuButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMenuButton {
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
pub struct IMenuButton_abi(
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
        idcommand: i32,
        lpbuttontext: super::super::Foundation::PWSTR,
        lptooltiptext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idcommand: i32,
        lpbuttontext: super::super::Foundation::PWSTR,
        lptooltiptext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
        bstate: super::super::Foundation::BOOL,
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
pub struct IMessageView(::windows::runtime::IUnknown);
impl IMessageView {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitleText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        psztitletext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            psztitletext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBodyText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszbodytext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszbodytext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetIcon(&self, id: IconIdentifier) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(id),
        )
        .ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMessageView {
    type Vtable = IMessageView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2163818868,
        64716,
        4562,
        [185, 145, 0, 192, 79, 142, 205, 120],
    );
}
impl ::std::convert::From<IMessageView> for ::windows::runtime::IUnknown {
    fn from(value: IMessageView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMessageView> for ::windows::runtime::IUnknown {
    fn from(value: &IMessageView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMessageView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMessageView {
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
pub struct IMessageView_abi(
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
        psztitletext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszbodytext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: IconIdentifier,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct INodeProperties(::windows::runtime::IUnknown);
impl INodeProperties {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        pdataobject: Param0,
        szpropertyname: Param1,
    ) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pdataobject.into_param().abi(),
            szpropertyname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INodeProperties {
    type Vtable = INodeProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        364662052,
        42274,
        17414,
        [170, 85, 7, 73, 83, 122, 104, 101],
    );
}
impl ::std::convert::From<INodeProperties> for ::windows::runtime::IUnknown {
    fn from(value: INodeProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INodeProperties> for ::windows::runtime::IUnknown {
    fn from(value: &INodeProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INodeProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INodeProperties {
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
pub struct INodeProperties_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdataobject: ::windows::runtime::RawPtr,
        szpropertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrproperty: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPropertySheetCallback(::windows::runtime::IUnknown);
impl IPropertySheetCallback {
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Controls::HPROPSHEETPAGE>,
    >(
        &self,
        hpage: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            hpage.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn RemovePage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Controls::HPROPSHEETPAGE>,
    >(
        &self,
        hpage: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            hpage.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySheetCallback {
    type Vtable = IPropertySheetCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2245944541,
        61217,
        4559,
        [162, 133, 0, 192, 79, 216, 219, 230],
    );
}
impl ::std::convert::From<IPropertySheetCallback> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySheetCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySheetCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySheetCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPropertySheetCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPropertySheetCallback
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
pub struct IPropertySheetCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Controls")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hpage: super::super::UI::Controls::HPROPSHEETPAGE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))] usize,
    #[cfg(feature = "Win32_UI_Controls")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hpage: super::super::UI::Controls::HPROPSHEETPAGE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPropertySheetProvider(::windows::runtime::IUnknown);
impl IPropertySheetProvider {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePropertySheet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        title: Param0,
        r#type: u8,
        cookie: isize,
        pidataobjectm: Param3,
        dwoptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            title.into_param().abi(),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(cookie),
            pidataobjectm.into_param().abi(),
            ::std::mem::transmute(dwoptions),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindPropertySheet<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IComponent>,
        Param2: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>,
    >(
        &self,
        hitem: isize,
        lpcomponent: Param1,
        lpdataobject: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hitem),
            lpcomponent.into_param().abi(),
            lpdataobject.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPrimaryPages<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        lpunknown: Param0,
        bcreatehandle: Param1,
        hnotifywindow: Param2,
        bscopepane: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lpunknown.into_param().abi(),
            bcreatehandle.into_param().abi(),
            hnotifywindow.into_param().abi(),
            bscopepane.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AddExtensionPages(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Show(&self, window: isize, page: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(window),
            ::std::mem::transmute(page),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySheetProvider {
    type Vtable = IPropertySheetProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2245944542,
        61217,
        4559,
        [162, 133, 0, 192, 79, 216, 219, 230],
    );
}
impl ::std::convert::From<IPropertySheetProvider> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySheetProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySheetProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySheetProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPropertySheetProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPropertySheetProvider
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
pub struct IPropertySheetProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        title: super::super::Foundation::PWSTR,
        r#type: u8,
        cookie: isize,
        pidataobjectm: ::windows::runtime::RawPtr,
        dwoptions: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hitem: isize,
        lpcomponent: ::windows::runtime::RawPtr,
        lpdataobject: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpunknown: ::windows::runtime::RawPtr,
        bcreatehandle: super::super::Foundation::BOOL,
        hnotifywindow: super::super::Foundation::HWND,
        bscopepane: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        window: isize,
        page: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRequiredExtensions(::windows::runtime::IUnknown);
impl IRequiredExtensions {
    pub unsafe fn EnableAllExtensions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetFirstExtension(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetNextExtension(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRequiredExtensions {
    type Vtable = IRequiredExtensions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1920478586,
        42144,
        4561,
        [175, 15, 0, 192, 79, 182, 221, 44],
    );
}
impl ::std::convert::From<IRequiredExtensions> for ::windows::runtime::IUnknown {
    fn from(value: IRequiredExtensions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRequiredExtensions> for ::windows::runtime::IUnknown {
    fn from(value: &IRequiredExtensions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRequiredExtensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRequiredExtensions {
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
pub struct IRequiredExtensions_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pextclsid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pextclsid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IResultData(::windows::runtime::IUnknown);
impl IResultData {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itemid),
            ::std::mem::transmute(ncol),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lparam: Param0,
    ) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lparam.into_param().abi(),
            &mut result__,
        )
        .from_abi::<isize>(result__)
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn ModifyItemState(
        &self,
        nindex: i32,
        itemid: isize,
        uadd: u32,
        uremove: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(itemid),
            ::std::mem::transmute(uadd),
            ::std::mem::transmute(uremove),
        )
        .ok()
    }
    pub unsafe fn ModifyViewStyle(
        &self,
        add: MMC_RESULT_VIEW_STYLE,
        remove: MMC_RESULT_VIEW_STYLE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(add),
            ::std::mem::transmute(remove),
        )
        .ok()
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lviewmode),
        )
        .ok()
    }
    pub unsafe fn GetViewMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itemid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncolumn),
            ::std::mem::transmute(dwsortoptions),
            luserparam.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescBarText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        desctext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            desctext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetItemCount(
        &self,
        nitemcount: i32,
        dwoptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nitemcount),
            ::std::mem::transmute(dwoptions),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResultData {
    type Vtable = IResultData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        836394912,
        57579,
        4559,
        [159, 33, 0, 170, 0, 60, 169, 246],
    );
}
impl ::std::convert::From<IResultData> for ::windows::runtime::IUnknown {
    fn from(value: IResultData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultData> for ::windows::runtime::IUnknown {
    fn from(value: &IResultData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResultData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResultData {
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
pub struct IResultData_abi(
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
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itemid: isize,
        ncol: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lparam: super::super::Foundation::LPARAM,
        pitemid: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *const RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nindex: i32,
        itemid: isize,
        uadd: u32,
        uremove: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        add: MMC_RESULT_VIEW_STYLE,
        remove: MMC_RESULT_VIEW_STYLE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lviewmode: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lviewmode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itemid: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desctext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nitemcount: i32,
        dwoptions: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IResultData2(::windows::runtime::IUnknown);
impl IResultData2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itemid),
            ::std::mem::transmute(ncol),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lparam: Param0,
    ) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            lparam.into_param().abi(),
            &mut result__,
        )
        .from_abi::<isize>(result__)
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
        )
        .ok()
    }
    pub unsafe fn ModifyItemState(
        &self,
        nindex: i32,
        itemid: isize,
        uadd: u32,
        uremove: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(itemid),
            ::std::mem::transmute(uadd),
            ::std::mem::transmute(uremove),
        )
        .ok()
    }
    pub unsafe fn ModifyViewStyle(
        &self,
        add: MMC_RESULT_VIEW_STYLE,
        remove: MMC_RESULT_VIEW_STYLE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(add),
            ::std::mem::transmute(remove),
        )
        .ok()
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lviewmode),
        )
        .ok()
    }
    pub unsafe fn GetViewMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itemid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncolumn),
            ::std::mem::transmute(dwsortoptions),
            luserparam.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescBarText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        desctext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            desctext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetItemCount(
        &self,
        nitemcount: i32,
        dwoptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nitemcount),
            ::std::mem::transmute(dwoptions),
        )
        .ok()
    }
    pub unsafe fn RenameResultItem(&self, itemid: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itemid),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResultData2 {
    type Vtable = IResultData2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        255254763,
        42993,
        19073,
        [190, 90, 146, 71, 247, 222, 75, 27],
    );
}
impl ::std::convert::From<IResultData2> for ::windows::runtime::IUnknown {
    fn from(value: IResultData2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultData2> for ::windows::runtime::IUnknown {
    fn from(value: &IResultData2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResultData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResultData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IResultData2> for IResultData {
    fn from(value: IResultData2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultData2> for IResultData {
    fn from(value: &IResultData2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResultData> for IResultData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResultData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResultData>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IResultData> for &IResultData2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IResultData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IResultData>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData2_abi(
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
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itemid: isize,
        ncol: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lparam: super::super::Foundation::LPARAM,
        pitemid: *mut isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *const RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: *mut RESULTDATAITEM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nindex: i32,
        itemid: isize,
        uadd: u32,
        uremove: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        add: MMC_RESULT_VIEW_STYLE,
        remove: MMC_RESULT_VIEW_STYLE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lviewmode: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lviewmode: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itemid: isize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        desctext: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nitemcount: i32,
        dwoptions: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itemid: isize,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IResultDataCompare(::windows::runtime::IUnknown);
impl IResultDataCompare {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        luserparam: Param0,
        cookiea: isize,
        cookieb: isize,
        pnresult: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            luserparam.into_param().abi(),
            ::std::mem::transmute(cookiea),
            ::std::mem::transmute(cookieb),
            ::std::mem::transmute(pnresult),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResultDataCompare {
    type Vtable = IResultDataCompare_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3895548498,
        31258,
        4560,
        [162, 210, 0, 192, 79, 217, 9, 221],
    );
}
impl ::std::convert::From<IResultDataCompare> for ::windows::runtime::IUnknown {
    fn from(value: IResultDataCompare) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultDataCompare> for ::windows::runtime::IUnknown {
    fn from(value: &IResultDataCompare) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResultDataCompare {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResultDataCompare {
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
pub struct IResultDataCompare_abi(
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
        luserparam: super::super::Foundation::LPARAM,
        cookiea: isize,
        cookieb: isize,
        pnresult: *mut i32,
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
pub struct IResultDataCompareEx(::windows::runtime::IUnknown);
impl IResultDataCompareEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare(&self, prdc: *const RDCOMPARE) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(prdc),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IResultDataCompareEx {
    type Vtable = IResultDataCompareEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2526229622,
        593,
        4563,
        [174, 176, 0, 192, 79, 142, 205, 120],
    );
}
impl ::std::convert::From<IResultDataCompareEx> for ::windows::runtime::IUnknown {
    fn from(value: IResultDataCompareEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultDataCompareEx> for ::windows::runtime::IUnknown {
    fn from(value: &IResultDataCompareEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResultDataCompareEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResultDataCompareEx {
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
pub struct IResultDataCompareEx_abi(
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
        prdc: *const RDCOMPARE,
        pnresult: *mut i32,
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
pub struct IResultOwnerData(::windows::runtime::IUnknown);
impl IResultOwnerData {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItem(
        &self,
        pfindinfo: *const RESULTFINDINFO,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfindinfo),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn CacheHint(
        &self,
        nstartindex: i32,
        nendindex: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nstartindex),
            ::std::mem::transmute(nendindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SortItems<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ncolumn),
            ::std::mem::transmute(dwsortoptions),
            luserparam.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IResultOwnerData {
    type Vtable = IResultOwnerData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2629015256,
        60035,
        4560,
        [174, 241, 0, 192, 79, 182, 221, 44],
    );
}
impl ::std::convert::From<IResultOwnerData> for ::windows::runtime::IUnknown {
    fn from(value: IResultOwnerData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IResultOwnerData> for ::windows::runtime::IUnknown {
    fn from(value: &IResultOwnerData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IResultOwnerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IResultOwnerData {
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
pub struct IResultOwnerData_abi(
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
        pfindinfo: *const RESULTFINDINFO,
        pnfoundindex: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nstartindex: i32,
        nendindex: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ncolumn: i32,
        dwsortoptions: u32,
        luserparam: super::super::Foundation::LPARAM,
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
pub struct ISnapinAbout(::windows::runtime::IUnknown);
impl ISnapinAbout {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapinDescription(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProvider(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapinVersion(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetSnapinImage(
        &self,
    ) -> ::windows::runtime::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ : < super::super::UI::WindowsAndMessaging:: HICON as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::UI::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetStaticFolderImage(
        &self,
        hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP,
        hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP,
        hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP,
        cmask: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hsmallimage),
            ::std::mem::transmute(hsmallimageopen),
            ::std::mem::transmute(hlargeimage),
            ::std::mem::transmute(cmask),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISnapinAbout {
    type Vtable = ISnapinAbout_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        306520204,
        41297,
        4560,
        [167, 215, 0, 192, 79, 217, 9, 221],
    );
}
impl ::std::convert::From<ISnapinAbout> for ::windows::runtime::IUnknown {
    fn from(value: ISnapinAbout) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinAbout> for ::windows::runtime::IUnknown {
    fn from(value: &ISnapinAbout) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISnapinAbout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISnapinAbout {
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
pub struct ISnapinAbout_abi(
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
        lpdescription: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpname: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpversion: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        happicon: *mut super::super::UI::WindowsAndMessaging::HICON,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP,
        hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP,
        hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP,
        cmask: *mut u32,
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
pub struct ISnapinHelp(::windows::runtime::IUnknown);
impl ISnapinHelp {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpTopic(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISnapinHelp {
    type Vtable = ISnapinHelp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2796640974,
        57177,
        4560,
        [167, 221, 0, 192, 79, 217, 9, 221],
    );
}
impl ::std::convert::From<ISnapinHelp> for ::windows::runtime::IUnknown {
    fn from(value: ISnapinHelp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinHelp> for ::windows::runtime::IUnknown {
    fn from(value: &ISnapinHelp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISnapinHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISnapinHelp {
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
pub struct ISnapinHelp_abi(
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
        lpcompiledhelpfile: *mut super::super::Foundation::PWSTR,
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
pub struct ISnapinHelp2(::windows::runtime::IUnknown);
impl ISnapinHelp2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpTopic(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkedTopics(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISnapinHelp2 {
    type Vtable = ISnapinHelp2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1214357520,
        8441,
        4562,
        [165, 16, 0, 192, 79, 182, 221, 44],
    );
}
impl ::std::convert::From<ISnapinHelp2> for ::windows::runtime::IUnknown {
    fn from(value: ISnapinHelp2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinHelp2> for ::windows::runtime::IUnknown {
    fn from(value: &ISnapinHelp2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISnapinHelp2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISnapinHelp2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ISnapinHelp2> for ISnapinHelp {
    fn from(value: ISnapinHelp2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinHelp2> for ISnapinHelp {
    fn from(value: &ISnapinHelp2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISnapinHelp> for ISnapinHelp2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISnapinHelp> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISnapinHelp>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISnapinHelp> for &ISnapinHelp2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISnapinHelp> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISnapinHelp>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp2_abi(
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
        lpcompiledhelpfile: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpcompiledhelpfiles: *mut super::super::Foundation::PWSTR,
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
pub struct ISnapinProperties(::windows::runtime::IUnknown);
impl ISnapinProperties {
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, Properties>>(
        &self,
        pproperties: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pproperties.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn QueryPropertyNames<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ISnapinPropertiesCallback>,
    >(
        &self,
        pcallback: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pcallback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn PropertiesChanged(
        &self,
        cproperties: i32,
        pproperties: *const MMC_SNAPIN_PROPERTY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cproperties),
            ::std::mem::transmute(pproperties),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISnapinProperties {
    type Vtable = ISnapinProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4152925609,
        18946,
        18487,
        [191, 137, 26, 111, 42, 2, 16, 16],
    );
}
impl ::std::convert::From<ISnapinProperties> for ::windows::runtime::IUnknown {
    fn from(value: ISnapinProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinProperties> for ::windows::runtime::IUnknown {
    fn from(value: &ISnapinProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISnapinProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISnapinProperties {
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
pub struct ISnapinProperties_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproperties: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cproperties: i32,
        pproperties: *const ::std::mem::ManuallyDrop<MMC_SNAPIN_PROPERTY>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
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
pub struct ISnapinPropertiesCallback(::windows::runtime::IUnknown);
impl ISnapinPropertiesCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPropertyName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpropname: Param0,
        dwflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszpropname.into_param().abi(),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISnapinPropertiesCallback {
    type Vtable = ISnapinPropertiesCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2769265381,
        32353,
        17899,
        [168, 212, 154, 7, 179, 232, 81, 168],
    );
}
impl ::std::convert::From<ISnapinPropertiesCallback> for ::windows::runtime::IUnknown {
    fn from(value: ISnapinPropertiesCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISnapinPropertiesCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ISnapinPropertiesCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISnapinPropertiesCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISnapinPropertiesCallback
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
pub struct ISnapinPropertiesCallback_abi(
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
        pszpropname: super::super::Foundation::PWSTR,
        dwflags: u32,
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
pub struct IStringTable(::windows::runtime::IUnknown);
impl IStringTable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszadd: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszadd.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(
        &self,
        stringid: u32,
        cchbuffer: u32,
        lpbuffer: super::super::Foundation::PWSTR,
        pcchout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(stringid),
            ::std::mem::transmute(cchbuffer),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(pcchout),
        )
        .ok()
    }
    pub unsafe fn GetStringLength(&self, stringid: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(stringid),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn DeleteString(&self, stringid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(stringid),
        )
        .ok()
    }
    pub unsafe fn DeleteAllStrings(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszfind: Param0,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            pszfind.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enumerate(&self) -> ::windows::runtime::Result<super::Com::IEnumString> {
        let mut result__: <super::Com::IEnumString as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::IEnumString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStringTable {
    type Vtable = IStringTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3728783268,
        3941,
        4562,
        [142, 37, 0, 192, 79, 142, 205, 120],
    );
}
impl ::std::convert::From<IStringTable> for ::windows::runtime::IUnknown {
    fn from(value: IStringTable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStringTable> for ::windows::runtime::IUnknown {
    fn from(value: &IStringTable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStringTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStringTable {
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
pub struct IStringTable_abi(
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
        pszadd: super::super::Foundation::PWSTR,
        pstringid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        stringid: u32,
        cchbuffer: u32,
        lpbuffer: super::super::Foundation::PWSTR,
        pcchout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        stringid: u32,
        pcchstring: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        stringid: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfind: super::super::Foundation::PWSTR,
        pstringid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IToolbar(::windows::runtime::IUnknown);
impl IToolbar {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddBitmap<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>,
    >(
        &self,
        nimages: i32,
        hbmp: Param1,
        cxsize: i32,
        cysize: i32,
        crmask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nimages),
            hbmp.into_param().abi(),
            ::std::mem::transmute(cxsize),
            ::std::mem::transmute(cysize),
            ::std::mem::transmute(crmask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddButtons(
        &self,
        nbuttons: i32,
        lpbuttons: *const MMCBUTTON,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nbuttons),
            ::std::mem::transmute(lpbuttons),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertButton(
        &self,
        nindex: i32,
        lpbutton: *const MMCBUTTON,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(lpbutton),
        )
        .ok()
    }
    pub unsafe fn DeleteButton(&self, nindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(nindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetButtonState(
        &self,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idcommand),
            ::std::mem::transmute(nstate),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
        bstate: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(idcommand),
            ::std::mem::transmute(nstate),
            bstate.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IToolbar {
    type Vtable = IToolbar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1125347001,
        54124,
        4559,
        [173, 188, 0, 170, 0, 168, 0, 51],
    );
}
impl ::std::convert::From<IToolbar> for ::windows::runtime::IUnknown {
    fn from(value: IToolbar) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IToolbar> for ::windows::runtime::IUnknown {
    fn from(value: &IToolbar) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IToolbar {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IToolbar {
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
pub struct IToolbar_abi(
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
        nimages: i32,
        hbmp: super::super::Graphics::Gdi::HBITMAP,
        cxsize: i32,
        cysize: i32,
        crmask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nbuttons: i32,
        lpbuttons: *const MMCBUTTON,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nindex: i32,
        lpbutton: *const MMCBUTTON,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nindex: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
        pstate: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        idcommand: i32,
        nstate: MMC_BUTTON_STATE,
        bstate: super::super::Foundation::BOOL,
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
pub struct IViewExtensionCallback(::windows::runtime::IUnknown);
impl IViewExtensionCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddView(
        &self,
        pextviewdata: *const MMC_EXT_VIEW_DATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pextviewdata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IViewExtensionCallback {
    type Vtable = IViewExtensionCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        886936202,
        30105,
        16869,
        [159, 94, 214, 188, 48, 98, 194, 218],
    );
}
impl ::std::convert::From<IViewExtensionCallback> for ::windows::runtime::IUnknown {
    fn from(value: IViewExtensionCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IViewExtensionCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IViewExtensionCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IViewExtensionCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IViewExtensionCallback
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
pub struct IViewExtensionCallback_abi(
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
        pextviewdata: *const MMC_EXT_VIEW_DATA,
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
pub struct IconIdentifier(pub i32);
pub const Icon_None: IconIdentifier = IconIdentifier(0i32);
pub const Icon_Error: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_Question: IconIdentifier = IconIdentifier(32514i32);
pub const Icon_Warning: IconIdentifier = IconIdentifier(32515i32);
pub const Icon_Information: IconIdentifier = IconIdentifier(32516i32);
pub const Icon_First: IconIdentifier = IconIdentifier(32513i32);
pub const Icon_Last: IconIdentifier = IconIdentifier(32516i32);
impl ::std::convert::From<i32> for IconIdentifier {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IconIdentifier {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl MENUBUTTONDATA {}
impl ::std::default::Default for MENUBUTTONDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MENUBUTTONDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MENUBUTTONDATA")
            .field("idCommand", &self.idCommand)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MENUBUTTONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.idCommand == other.idCommand && self.x == other.x && self.y == other.y
    }
}
impl ::std::cmp::Eq for MENUBUTTONDATA {}
unsafe impl ::windows::runtime::Abi for MENUBUTTONDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: super::super::Foundation::PWSTR,
    pub lpTooltipText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MMCBUTTON {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMCBUTTON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMCBUTTON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMCBUTTON")
            .field("nBitmap", &self.nBitmap)
            .field("idCommand", &self.idCommand)
            .field("fsState", &self.fsState)
            .field("fsType", &self.fsType)
            .field("lpButtonText", &self.lpButtonText)
            .field("lpTooltipText", &self.lpTooltipText)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMCBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.nBitmap == other.nBitmap
            && self.idCommand == other.idCommand
            && self.fsState == other.fsState
            && self.fsType == other.fsType
            && self.lpButtonText == other.lpButtonText
            && self.lpTooltipText == other.lpTooltipText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMCBUTTON {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMCBUTTON {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMCLV_AUTO: i32 = -1i32;
pub const MMCLV_NOICON: i32 = -1i32;
pub const MMCLV_NOPARAM: i32 = -2i32;
pub const MMCLV_NOPTR: u32 = 0u32;
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub const MMCVersionInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3607026461,
    53025,
    19417,
    [175, 59, 197, 70, 142, 156, 102, 132],
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
pub struct MMC_ACTION_TYPE(pub i32);
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = MMC_ACTION_TYPE(-1i32);
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = MMC_ACTION_TYPE(0i32);
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = MMC_ACTION_TYPE(1i32);
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = MMC_ACTION_TYPE(2i32);
impl ::std::convert::From<i32> for MMC_ACTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_ACTION_TYPE {
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
pub struct MMC_BUTTON_STATE(pub i32);
pub const ENABLED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(1i32);
pub const CHECKED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(2i32);
pub const HIDDEN: MMC_BUTTON_STATE = MMC_BUTTON_STATE(4i32);
pub const INDETERMINATE: MMC_BUTTON_STATE = MMC_BUTTON_STATE(8i32);
pub const BUTTONPRESSED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(16i32);
impl ::std::convert::From<i32> for MMC_BUTTON_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_BUTTON_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl MMC_COLUMN_DATA {}
impl ::std::default::Default for MMC_COLUMN_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMC_COLUMN_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_COLUMN_DATA")
            .field("nColIndex", &self.nColIndex)
            .field("dwFlags", &self.dwFlags)
            .field("nWidth", &self.nWidth)
            .field("ulReserved", &self.ulReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMC_COLUMN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.nColIndex == other.nColIndex
            && self.dwFlags == other.dwFlags
            && self.nWidth == other.nWidth
            && self.ulReserved == other.ulReserved
    }
}
impl ::std::cmp::Eq for MMC_COLUMN_DATA {}
unsafe impl ::windows::runtime::Abi for MMC_COLUMN_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl MMC_COLUMN_SET_DATA {}
impl ::std::default::Default for MMC_COLUMN_SET_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMC_COLUMN_SET_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_COLUMN_SET_DATA")
            .field("cbSize", &self.cbSize)
            .field("nNumCols", &self.nNumCols)
            .field("pColData", &self.pColData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMC_COLUMN_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.nNumCols == other.nNumCols
            && self.pColData == other.pColData
    }
}
impl ::std::cmp::Eq for MMC_COLUMN_SET_DATA {}
unsafe impl ::windows::runtime::Abi for MMC_COLUMN_SET_DATA {
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
pub struct MMC_CONSOLE_VERB(pub i32);
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(0i32);
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32769i32);
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32770i32);
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32771i32);
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32772i32);
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32773i32);
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32774i32);
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32775i32);
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32777i32);
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
impl ::std::convert::From<i32> for MMC_CONSOLE_VERB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_CONSOLE_VERB {
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
pub struct MMC_CONTROL_TYPE(pub i32);
pub const TOOLBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(0i32);
pub const MENUBUTTON: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(1i32);
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(2i32);
impl ::std::convert::From<i32> for MMC_CONTROL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_CONTROL_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_EXPANDSYNC_STRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_EXPANDSYNC_STRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_EXPANDSYNC_STRUCT")
            .field("bHandled", &self.bHandled)
            .field("bExpanding", &self.bExpanding)
            .field("hItem", &self.hItem)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_EXPANDSYNC_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.bHandled == other.bHandled
            && self.bExpanding == other.bExpanding
            && self.hItem == other.hItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_EXPANDSYNC_STRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: ::windows::runtime::GUID,
    pub pszURL: super::super::Foundation::PWSTR,
    pub pszViewTitle: super::super::Foundation::PWSTR,
    pub pszTooltipText: super::super::Foundation::PWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_EXT_VIEW_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_EXT_VIEW_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_EXT_VIEW_DATA")
            .field("viewID", &self.viewID)
            .field("pszURL", &self.pszURL)
            .field("pszViewTitle", &self.pszViewTitle)
            .field("pszTooltipText", &self.pszTooltipText)
            .field("bReplacesDefaultView", &self.bReplacesDefaultView)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_EXT_VIEW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.viewID == other.viewID
            && self.pszURL == other.pszURL
            && self.pszViewTitle == other.pszViewTitle
            && self.pszTooltipText == other.pszTooltipText
            && self.bReplacesDefaultView == other.bReplacesDefaultView
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_EXT_VIEW_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_FILTERDATA {
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_FILTERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_FILTERDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_FILTERDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_FILTERDATA")
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("lValue", &self.lValue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_FILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText
            && self.cchTextMax == other.cchTextMax
            && self.lValue == other.lValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_FILTERDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_FILTERDATA {
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
pub struct MMC_FILTER_CHANGE_CODE(pub i32);
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(0i32);
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(1i32);
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(2i32);
impl ::std::convert::From<i32> for MMC_FILTER_CHANGE_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_FILTER_CHANGE_CODE {
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
pub struct MMC_FILTER_TYPE(pub i32);
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(0i32);
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(1i32);
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = MMC_FILTER_TYPE(32768i32);
impl ::std::convert::From<i32> for MMC_FILTER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_FILTER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_IMAGECALLBACK: i32 = -1i32;
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: super::super::Foundation::PWSTR,
    pub szButtonText: super::super::Foundation::PWSTR,
    pub nCommandID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_LISTPAD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_LISTPAD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_LISTPAD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_LISTPAD_INFO")
            .field("szTitle", &self.szTitle)
            .field("szButtonText", &self.szButtonText)
            .field("nCommandID", &self.nCommandID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_LISTPAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.szTitle == other.szTitle
            && self.szButtonText == other.szButtonText
            && self.nCommandID == other.nCommandID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_LISTPAD_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_LISTPAD_INFO {
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
pub struct MMC_MENU_COMMAND_IDS(pub i32);
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = MMC_MENU_COMMAND_IDS(-1i32);
impl ::std::convert::From<i32> for MMC_MENU_COMMAND_IDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_MENU_COMMAND_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MMC_NOTIFY_TYPE(pub i32);
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32769i32);
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32770i32);
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32771i32);
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32772i32);
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32773i32);
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32774i32);
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32775i32);
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32776i32);
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32777i32);
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32778i32);
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32779i32);
pub const MMCN_HELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32780i32);
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32781i32);
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32782i32);
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32783i32);
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32784i32);
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32785i32);
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32786i32);
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32787i32);
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32788i32);
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32789i32);
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32790i32);
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32791i32);
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32792i32);
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32793i32);
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32794i32);
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32795i32);
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32796i32);
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32797i32);
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32798i32);
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32799i32);
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32800i32);
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32801i32);
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32802i32);
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32803i32);
impl ::std::convert::From<i32> for MMC_NOTIFY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_NOTIFY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MMC_PROPERTY_ACTION(pub i32);
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(1i32);
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(2i32);
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(3i32);
impl ::std::convert::From<i32> for MMC_PROPERTY_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_PROPERTY_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
pub const MMC_PROP_PERSIST: u32 = 8u32;
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
pub const MMC_PSO_HASHELP: u32 = 2u32;
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: super::super::Foundation::PWSTR,
    pub lViewOptions: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_RESTORE_VIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_RESTORE_VIEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_RESTORE_VIEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_RESTORE_VIEW")
            .field("dwSize", &self.dwSize)
            .field("cookie", &self.cookie)
            .field("pViewType", &self.pViewType)
            .field("lViewOptions", &self.lViewOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_RESTORE_VIEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.cookie == other.cookie
            && self.pViewType == other.pViewType
            && self.lViewOptions == other.lViewOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_RESTORE_VIEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_RESTORE_VIEW {
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
pub struct MMC_RESULT_VIEW_STYLE(pub i32);
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(1i32);
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(2i32);
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(4i32);
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(8i32);
impl ::std::convert::From<i32> for MMC_RESULT_VIEW_STYLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_RESULT_VIEW_STYLE {
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
pub struct MMC_SCOPE_ITEM_STATE(pub i32);
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(1i32);
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(2i32);
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(3i32);
impl ::std::convert::From<i32> for MMC_SCOPE_ITEM_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_SCOPE_ITEM_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: super::super::Foundation::PWSTR,
    pub varValue: super::OleAutomation::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl MMC_SNAPIN_PROPERTY {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for MMC_SNAPIN_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for MMC_SNAPIN_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for MMC_SNAPIN_PROPERTY {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for MMC_SNAPIN_PROPERTY {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl MMC_SORT_DATA {}
impl ::std::default::Default for MMC_SORT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMC_SORT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_SORT_DATA")
            .field("nColIndex", &self.nColIndex)
            .field("dwSortOptions", &self.dwSortOptions)
            .field("ulReserved", &self.ulReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMC_SORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.nColIndex == other.nColIndex
            && self.dwSortOptions == other.dwSortOptions
            && self.ulReserved == other.ulReserved
    }
}
impl ::std::cmp::Eq for MMC_SORT_DATA {}
unsafe impl ::windows::runtime::Abi for MMC_SORT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl MMC_SORT_SET_DATA {}
impl ::std::default::Default for MMC_SORT_SET_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMC_SORT_SET_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_SORT_SET_DATA")
            .field("cbSize", &self.cbSize)
            .field("nNumItems", &self.nNumItems)
            .field("pSortData", &self.pSortData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMC_SORT_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.nNumItems == other.nNumItems
            && self.pSortData == other.pSortData
    }
}
impl ::std::cmp::Eq for MMC_SORT_SET_DATA {}
unsafe impl ::windows::runtime::Abi for MMC_SORT_SET_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: super::super::Foundation::PWSTR,
    pub szHelpString: super::super::Foundation::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: super::super::Foundation::PWSTR,
    pub szScript: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: super::super::Foundation::PWSTR,
    pub szMouseOffBitmap: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK_DISPLAY_BITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK_DISPLAY_BITMAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_TASK_DISPLAY_BITMAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_TASK_DISPLAY_BITMAP")
            .field("szMouseOverBitmap", &self.szMouseOverBitmap)
            .field("szMouseOffBitmap", &self.szMouseOffBitmap)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK_DISPLAY_BITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.szMouseOverBitmap == other.szMouseOverBitmap
            && self.szMouseOffBitmap == other.szMouseOffBitmap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK_DISPLAY_BITMAP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK_DISPLAY_BITMAP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK_DISPLAY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK_DISPLAY_OBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK_DISPLAY_OBJECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK_DISPLAY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK_DISPLAY_OBJECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK_DISPLAY_OBJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK_DISPLAY_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK_DISPLAY_OBJECT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK_DISPLAY_OBJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK_DISPLAY_OBJECT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: super::super::Foundation::PWSTR,
    pub szURLtoEOT: super::super::Foundation::PWSTR,
    pub szSymbolString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MMC_TASK_DISPLAY_SYMBOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MMC_TASK_DISPLAY_SYMBOL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MMC_TASK_DISPLAY_SYMBOL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_TASK_DISPLAY_SYMBOL")
            .field("szFontFamilyName", &self.szFontFamilyName)
            .field("szURLtoEOT", &self.szURLtoEOT)
            .field("szSymbolString", &self.szSymbolString)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MMC_TASK_DISPLAY_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.szFontFamilyName == other.szFontFamilyName
            && self.szURLtoEOT == other.szURLtoEOT
            && self.szSymbolString == other.szSymbolString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MMC_TASK_DISPLAY_SYMBOL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MMC_TASK_DISPLAY_SYMBOL {
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
pub struct MMC_TASK_DISPLAY_TYPE(pub i32);
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(0i32);
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(1i32);
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(2i32);
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(3i32);
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(4i32);
impl ::std::convert::From<i32> for MMC_TASK_DISPLAY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_TASK_DISPLAY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_VER: u32 = 512u32;
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MMC_VIEW_TYPE(pub i32);
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = MMC_VIEW_TYPE(0i32);
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = MMC_VIEW_TYPE(1i32);
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = MMC_VIEW_TYPE(2i32);
impl ::std::convert::From<i32> for MMC_VIEW_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MMC_VIEW_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl MMC_VISIBLE_COLUMNS {}
impl ::std::default::Default for MMC_VISIBLE_COLUMNS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMC_VISIBLE_COLUMNS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MMC_VISIBLE_COLUMNS")
            .field("nVisibleColumns", &self.nVisibleColumns)
            .field("rgVisibleCols", &self.rgVisibleCols)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMC_VISIBLE_COLUMNS {
    fn eq(&self, other: &Self) -> bool {
        self.nVisibleColumns == other.nVisibleColumns && self.rgVisibleCols == other.rgVisibleCols
    }
}
impl ::std::cmp::Eq for MMC_VISIBLE_COLUMNS {}
unsafe impl ::windows::runtime::Abi for MMC_VISIBLE_COLUMNS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MenuItem(::windows::runtime::IUnknown);
impl MenuItem {
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
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn LanguageIndependentName(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn LanguageIndependentPath(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Execute(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for MenuItem {
    type Vtable = MenuItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        24705745,
        45921,
        19239,
        [150, 173, 103, 197, 126, 191, 46, 29],
    );
}
impl ::std::convert::From<MenuItem> for ::windows::runtime::IUnknown {
    fn from(value: MenuItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MenuItem> for ::windows::runtime::IUnknown {
    fn from(value: &MenuItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MenuItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MenuItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<MenuItem> for super::OleAutomation::IDispatch {
    fn from(value: MenuItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&MenuItem> for super::OleAutomation::IDispatch {
    fn from(value: &MenuItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for MenuItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &MenuItem {
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
pub struct MenuItem_abi(
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
        displayname: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        languageindependentname: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        path: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        languageindependentpath: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enabled: *mut super::super::Foundation::BOOL,
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
pub struct Node(::windows::runtime::IUnknown);
impl Node {
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
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Property<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            propertyname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Bookmark(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScopeNode(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Nodetype(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Node {
    type Vtable = Node_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4162770944,
        30777,
        17479,
        [148, 93, 142, 21, 218, 89, 202, 85],
    );
}
impl ::std::convert::From<Node> for ::windows::runtime::IUnknown {
    fn from(value: Node) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Node> for ::windows::runtime::IUnknown {
    fn from(value: &Node) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Node {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Node {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Node> for super::OleAutomation::IDispatch {
    fn from(value: Node) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Node> for super::OleAutomation::IDispatch {
    fn from(value: &Node) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Node {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Node {
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
pub struct Node_abi(
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
        name: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        propertyvalue: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bookmark: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isscopenode: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodetype: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Nodes(::windows::runtime::IUnknown);
impl Nodes {
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
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Nodes {
    type Vtable = Nodes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        825950687,
        45615,
        19778,
        [177, 184, 72, 60, 220, 245, 29, 53],
    );
}
impl ::std::convert::From<Nodes> for ::windows::runtime::IUnknown {
    fn from(value: Nodes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Nodes> for ::windows::runtime::IUnknown {
    fn from(value: &Nodes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Nodes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Nodes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Nodes> for super::OleAutomation::IDispatch {
    fn from(value: Nodes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Nodes> for super::OleAutomation::IDispatch {
    fn from(value: &Nodes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Nodes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Nodes {
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
pub struct Nodes_abi(
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
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        node: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Properties(::windows::runtime::IUnknown);
impl Properties {
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
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<Property> {
        let mut result__: <Property as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            name.into_param().abi(),
            &mut result__,
        )
        .from_abi::<Property>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            name.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for Properties {
    type Vtable = Properties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        679914434,
        42021,
        17074,
        [145, 198, 226, 92, 14, 4, 88, 28],
    );
}
impl ::std::convert::From<Properties> for ::windows::runtime::IUnknown {
    fn from(value: Properties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Properties> for ::windows::runtime::IUnknown {
    fn from(value: &Properties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Properties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Properties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Properties> for super::OleAutomation::IDispatch {
    fn from(value: Properties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Properties> for super::OleAutomation::IDispatch {
    fn from(value: &Properties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Properties {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Properties {
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
pub struct Properties_abi(
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
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        property: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct Property(::windows::runtime::IUnknown);
impl Property {
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
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::OleAutomation::VARIANT> {
        let mut result__: <super::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
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
    pub unsafe fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            value.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Property {
    type Vtable = Property_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1174455205,
        58113,
        16856,
        [182, 208, 239, 46, 66, 18, 224, 202],
    );
}
impl ::std::convert::From<Property> for ::windows::runtime::IUnknown {
    fn from(value: Property) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Property> for ::windows::runtime::IUnknown {
    fn from(value: &Property) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Property {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Property {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Property> for super::OleAutomation::IDispatch {
    fn from(value: Property) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Property> for super::OleAutomation::IDispatch {
    fn from(value: &Property) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Property {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Property {
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
pub struct Property_abi(
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
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
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
        value: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
);
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
#[cfg(feature = "Win32_Foundation")]
impl RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RDCOMPARE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RDCOMPARE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RDCOMPARE")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("nColumn", &self.nColumn)
            .field("lUserParam", &self.lUserParam)
            .field("prdch1", &self.prdch1)
            .field("prdch2", &self.prdch2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RDCOMPARE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.nColumn == other.nColumn
            && self.lUserParam == other.lUserParam
            && self.prdch1 == other.prdch1
            && self.prdch2 == other.prdch2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RDCOMPARE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RDITEMHDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RDITEMHDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RDITEMHDR")
            .field("dwFlags", &self.dwFlags)
            .field("cookie", &self.cookie)
            .field("lpReserved", &self.lpReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RDITEMHDR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.cookie == other.cookie
            && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RDITEMHDR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RDI_IMAGE: u32 = 4u32;
pub const RDI_INDENT: u32 = 64u32;
pub const RDI_INDEX: u32 = 32u32;
pub const RDI_PARAM: u32 = 16u32;
pub const RDI_STATE: u32 = 8u32;
pub const RDI_STR: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESULTDATAITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESULTDATAITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESULTDATAITEM")
            .field("mask", &self.mask)
            .field("bScopeItem", &self.bScopeItem)
            .field("itemID", &self.itemID)
            .field("nIndex", &self.nIndex)
            .field("nCol", &self.nCol)
            .field("str", &self.str)
            .field("nImage", &self.nImage)
            .field("nState", &self.nState)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESULTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.bScopeItem == other.bScopeItem
            && self.itemID == other.itemID
            && self.nIndex == other.nIndex
            && self.nCol == other.nCol
            && self.str == other.str
            && self.nImage == other.nImage
            && self.nState == other.nState
            && self.lParam == other.lParam
            && self.iIndent == other.iIndent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESULTDATAITEM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTFINDINFO {
    pub psz: super::super::Foundation::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl RESULTFINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESULTFINDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESULTFINDINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RESULTFINDINFO")
            .field("psz", &self.psz)
            .field("nStart", &self.nStart)
            .field("dwOptions", &self.dwOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESULTFINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.psz == other.psz && self.nStart == other.nStart && self.dwOptions == other.dwOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESULTFINDINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESULTFINDINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: super::super::Foundation::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RESULT_VIEW_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESULT_VIEW_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESULT_VIEW_TYPE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESULT_VIEW_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESULT_VIEW_TYPE_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: ::std::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO_0_1>,
}
#[cfg(feature = "Win32_Foundation")]
impl RESULT_VIEW_TYPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESULT_VIEW_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESULT_VIEW_TYPE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESULT_VIEW_TYPE_INFO_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl RESULT_VIEW_TYPE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RESULT_VIEW_TYPE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("dwHTMLOptions", &self.dwHTMLOptions)
            .field("pstrURL", &self.pstrURL)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwHTMLOptions == other.dwHTMLOptions && self.pstrURL == other.pstrURL
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RESULT_VIEW_TYPE_INFO_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl RESULT_VIEW_TYPE_INFO_0_1 {}
impl ::std::default::Default for RESULT_VIEW_TYPE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field("dwOCXOptions", &self.dwOCXOptions)
            .field("pUnkControl", &self.pUnkControl)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwOCXOptions == other.dwOCXOptions && self.pUnkControl == other.pUnkControl
    }
}
impl ::std::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_1 {}
unsafe impl ::windows::runtime::Abi for RESULT_VIEW_TYPE_INFO_0_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const RFI_PARTIAL: u32 = 1u32;
pub const RFI_WRAP: u32 = 2u32;
pub const RSI_DESCENDING: u32 = 1u32;
pub const RSI_NOSORTICON: u32 = 2u32;
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: super::super::Foundation::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCOPEDATAITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCOPEDATAITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCOPEDATAITEM")
            .field("mask", &self.mask)
            .field("displayname", &self.displayname)
            .field("nImage", &self.nImage)
            .field("nOpenImage", &self.nOpenImage)
            .field("nState", &self.nState)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("relativeID", &self.relativeID)
            .field("ID", &self.ID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCOPEDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.displayname == other.displayname
            && self.nImage == other.nImage
            && self.nOpenImage == other.nOpenImage
            && self.nState == other.nState
            && self.cChildren == other.cChildren
            && self.lParam == other.lParam
            && self.relativeID == other.relativeID
            && self.ID == other.ID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCOPEDATAITEM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl SColumnSetID {}
impl ::std::default::Default for SColumnSetID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SColumnSetID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SColumnSetID")
            .field("dwFlags", &self.dwFlags)
            .field("cBytes", &self.cBytes)
            .field("id", &self.id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SColumnSetID {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::std::cmp::Eq for SColumnSetID {}
unsafe impl ::windows::runtime::Abi for SColumnSetID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SDI_CHILDREN: u32 = 64u32;
pub const SDI_FIRST: u32 = 134217728u32;
pub const SDI_IMAGE: u32 = 4u32;
pub const SDI_NEXT: u32 = 536870912u32;
pub const SDI_OPENIMAGE: u32 = 8u32;
pub const SDI_PARAM: u32 = 32u32;
pub const SDI_PARENT: u32 = 0u32;
pub const SDI_PREVIOUS: u32 = 268435456u32;
pub const SDI_STATE: u32 = 16u32;
pub const SDI_STR: u32 = 2u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [::std::option::Option<super::Com::IDataObject>; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for SMMCDataObjects {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for SMMCDataObjects {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for SMMCDataObjects {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [::windows::runtime::GUID; 1],
}
impl SMMCObjectTypes {}
impl ::std::default::Default for SMMCObjectTypes {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SMMCObjectTypes {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SMMCObjectTypes")
            .field("count", &self.count)
            .field("guid", &self.guid)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SMMCObjectTypes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.guid == other.guid
    }
}
impl ::std::cmp::Eq for SMMCObjectTypes {}
unsafe impl ::windows::runtime::Abi for SMMCObjectTypes {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl SNodeID {}
impl ::std::default::Default for SNodeID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SNodeID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SNodeID")
            .field("cBytes", &self.cBytes)
            .field("id", &self.id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SNodeID {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::std::cmp::Eq for SNodeID {}
unsafe impl ::windows::runtime::Abi for SNodeID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl SNodeID2 {}
impl ::std::default::Default for SNodeID2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SNodeID2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SNodeID2")
            .field("dwFlags", &self.dwFlags)
            .field("cBytes", &self.cBytes)
            .field("id", &self.id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SNodeID2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::std::cmp::Eq for SNodeID2 {}
unsafe impl ::windows::runtime::Abi for SNodeID2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScopeNamespace(::windows::runtime::IUnknown);
impl ScopeNamespace {
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
    pub unsafe fn GetParent<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn GetChild<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn GetNext<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn GetRoot(&self) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn Expand<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ScopeNamespace {
    type Vtable = ScopeNamespace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3954919644,
        6715,
        19846,
        [183, 134, 194, 27, 40, 56, 144, 18],
    );
}
impl ::std::convert::From<ScopeNamespace> for ::windows::runtime::IUnknown {
    fn from(value: ScopeNamespace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ScopeNamespace> for ::windows::runtime::IUnknown {
    fn from(value: &ScopeNamespace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ScopeNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ScopeNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ScopeNamespace> for super::OleAutomation::IDispatch {
    fn from(value: ScopeNamespace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ScopeNamespace> for super::OleAutomation::IDispatch {
    fn from(value: &ScopeNamespace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for ScopeNamespace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &ScopeNamespace {
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
pub struct ScopeNamespace_abi(
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
        node: ::windows::runtime::RawPtr,
        parent: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
        child: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
        next: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        root: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SnapIn(::windows::runtime::IUnknown);
impl SnapIn {
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
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Vendor(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Extensions(&self) -> ::windows::runtime::Result<Extensions> {
        let mut result__: <Extensions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Extensions>(result__)
    }
    pub unsafe fn SnapinCLSID(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows::runtime::Result<Properties> {
        let mut result__: <Properties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Properties>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        enable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            enable.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for SnapIn {
    type Vtable = SnapIn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1005129974,
        13401,
        18886,
        [161, 187, 65, 230, 190, 157, 243, 234],
    );
}
impl ::std::convert::From<SnapIn> for ::windows::runtime::IUnknown {
    fn from(value: SnapIn) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SnapIn> for ::windows::runtime::IUnknown {
    fn from(value: &SnapIn) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SnapIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SnapIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<SnapIn> for super::OleAutomation::IDispatch {
    fn from(value: SnapIn) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&SnapIn> for super::OleAutomation::IDispatch {
    fn from(value: &SnapIn) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for SnapIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &SnapIn {
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
pub struct SnapIn_abi(
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
        name: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        vendor: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        version: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        extensions: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapinclsid: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        properties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        enable: super::super::Foundation::BOOL,
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
pub struct SnapIns(::windows::runtime::IUnknown);
impl SnapIns {
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
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<SnapIn> {
        let mut result__: <SnapIn as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<SnapIn>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
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
    pub unsafe fn Add<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
        Param2: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        snapinnameorclsid: Param0,
        parentsnapin: Param1,
        properties: Param2,
    ) -> ::windows::runtime::Result<SnapIn> {
        let mut result__: <SnapIn as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            snapinnameorclsid.into_param().abi(),
            parentsnapin.into_param().abi(),
            properties.into_param().abi(),
            &mut result__,
        )
        .from_abi::<SnapIn>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, SnapIn>>(
        &self,
        snapin: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            snapin.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for SnapIns {
    type Vtable = SnapIns_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        787734045,
        45354,
        18897,
        [146, 197, 11, 0, 121, 135, 104, 241],
    );
}
impl ::std::convert::From<SnapIns> for ::windows::runtime::IUnknown {
    fn from(value: SnapIns) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SnapIns> for ::windows::runtime::IUnknown {
    fn from(value: &SnapIns) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SnapIns {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SnapIns {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<SnapIns> for super::OleAutomation::IDispatch {
    fn from(value: SnapIns) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&SnapIns> for super::OleAutomation::IDispatch {
    fn from(value: &SnapIns) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for SnapIns {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &SnapIns {
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
pub struct SnapIns_abi(
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
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        snapin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapinnameorclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        parentsnapin: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        properties: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        snapin: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        snapin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct View(::windows::runtime::IUnknown);
impl View {
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
    pub unsafe fn ActiveScopeNode(&self) -> ::windows::runtime::Result<Node> {
        let mut result__: <Node as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Node>(result__)
    }
    pub unsafe fn SetActiveScopeNode<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Selection(&self) -> ::windows::runtime::Result<Nodes> {
        let mut result__: <Nodes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Nodes>(result__)
    }
    pub unsafe fn ListItems(&self) -> ::windows::runtime::Result<Nodes> {
        let mut result__: <Nodes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Nodes>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SnapinScopeObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn SnapinSelectionObject(
        &self,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
    pub unsafe fn Is<'a, Param0: ::windows::runtime::IntoParam<'a, View>>(
        &self,
        view: Param0,
    ) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<Document> {
        let mut result__: <Document as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Document>(result__)
    }
    pub unsafe fn SelectAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Select<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Deselect<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSelected<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn DisplayScopeNodePropertySheet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DisplaySelectionPropertySheet(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn CopyScopeNode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CopySelection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn DeleteScopeNode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DeleteSelection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn RenameScopeNode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        newname: Param0,
        scopenode: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            newname.into_param().abi(),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameSelectedItem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        newname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            newname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn ScopeNodeContextMenu<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<ContextMenu> {
        let mut result__: <ContextMenu as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ContextMenu>(result__)
    }
    pub unsafe fn SelectionContextMenu(&self) -> ::windows::runtime::Result<ContextMenu> {
        let mut result__: <ContextMenu as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ContextMenu>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn RefreshScopeNode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        scopenode: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RefreshSelection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteSelectionMenuItem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        menuitempath: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            menuitempath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn ExecuteScopeNodeMenuItem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::OleAutomation::VARIANT>,
    >(
        &self,
        menuitempath: Param0,
        scopenode: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            menuitempath.into_param().abi(),
            scopenode.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteShellCommand<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        command: Param0,
        directory: Param1,
        parameters: Param2,
        windowstate: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            command.into_param().abi(),
            directory.into_param().abi(),
            parameters.into_param().abi(),
            windowstate.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Frame(&self) -> ::windows::runtime::Result<Frame> {
        let mut result__: <Frame as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Frame>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScopeTreeVisible(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScopeTreeVisible<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        visible: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            visible.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Back(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Forward(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusBarText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        statusbartext: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            statusbartext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Memento(&self) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ViewMemento<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        memento: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            memento.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Columns(&self) -> ::windows::runtime::Result<Columns> {
        let mut result__: <Columns as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Columns>(result__)
    }
    pub unsafe fn CellContents<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
        column: i32,
    ) -> ::windows::runtime::Result<*mut u16> {
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            ::std::mem::transmute(column),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExportList<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        file: Param0,
        exportoptions: _ExportListOptions,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            file.into_param().abi(),
            ::std::mem::transmute(exportoptions),
        )
        .ok()
    }
    pub unsafe fn ListViewMode(&self) -> ::windows::runtime::Result<_ListViewMode> {
        let mut result__: <_ListViewMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<_ListViewMode>(result__)
    }
    pub unsafe fn SetListViewMode(&self, mode: _ListViewMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn ControlObject(
        &self,
    ) -> ::windows::runtime::Result<super::OleAutomation::IDispatch> {
        let mut result__: <super::OleAutomation::IDispatch as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::OleAutomation::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for View {
    type Vtable = View_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1862020514,
        45964,
        17790,
        [154, 187, 237, 45, 24, 155, 140, 56],
    );
}
impl ::std::convert::From<View> for ::windows::runtime::IUnknown {
    fn from(value: View) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&View> for ::windows::runtime::IUnknown {
    fn from(value: &View) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for View {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &View {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<View> for super::OleAutomation::IDispatch {
    fn from(value: View) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&View> for super::OleAutomation::IDispatch {
    fn from(value: &View) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for View {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &View {
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
pub struct View_abi(
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
        node: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodes: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        nodes: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        scopenodeobject: *mut ::windows::runtime::RawPtr,
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
        selectionobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
        thesame: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
        isselected: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        newname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
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
        newname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
        contextmenu: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        contextmenu: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        menuitempath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        menuitempath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        scopenode: ::std::mem::ManuallyDrop<super::OleAutomation::VARIANT>,
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
        command: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        directory: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        parameters: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        windowstate: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        frame: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visible: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visible: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        statusbartext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memento: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memento: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        columns: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
        column: i32,
        cellcontents: *mut *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        exportoptions: _ExportListOptions,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: *mut _ListViewMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: _ListViewMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        control: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Views(::windows::runtime::IUnknown);
impl Views {
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
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<View> {
        let mut result__: <View as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<View>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, Node>>(
        &self,
        node: Param0,
        viewoptions: _ViewOptions,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            node.into_param().abi(),
            ::std::mem::transmute(viewoptions),
        )
        .ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for Views {
    type Vtable = Views_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3602432669,
        41471,
        19826,
        [170, 176, 227, 129, 233, 185, 51, 141],
    );
}
impl ::std::convert::From<Views> for ::windows::runtime::IUnknown {
    fn from(value: Views) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Views> for ::windows::runtime::IUnknown {
    fn from(value: &Views) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Views {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Views {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<Views> for super::OleAutomation::IDispatch {
    fn from(value: Views) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&Views> for super::OleAutomation::IDispatch {
    fn from(value: &Views) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for Views {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &Views {
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
pub struct Views_abi(
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
        index: i32,
        view: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        node: ::windows::runtime::RawPtr,
        viewoptions: _ViewOptions,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        retval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _AppEvents(::windows::runtime::IUnknown);
impl _AppEvents {
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
    pub unsafe fn OnQuit<'a, Param0: ::windows::runtime::IntoParam<'a, _Application>>(
        &self,
        application: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            application.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDocumentOpen<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, Document>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        document: Param0,
        new: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            document.into_param().abi(),
            new.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnDocumentClose<'a, Param0: ::windows::runtime::IntoParam<'a, Document>>(
        &self,
        document: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            document.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnSnapInAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, Document>,
        Param1: ::windows::runtime::IntoParam<'a, SnapIn>,
    >(
        &self,
        document: Param0,
        snapin: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            document.into_param().abi(),
            snapin.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnSnapInRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, Document>,
        Param1: ::windows::runtime::IntoParam<'a, SnapIn>,
    >(
        &self,
        document: Param0,
        snapin: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            document.into_param().abi(),
            snapin.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnNewView<'a, Param0: ::windows::runtime::IntoParam<'a, View>>(
        &self,
        view: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnViewClose<'a, Param0: ::windows::runtime::IntoParam<'a, View>>(
        &self,
        view: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnViewChange<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, View>,
        Param1: ::windows::runtime::IntoParam<'a, Node>,
    >(
        &self,
        view: Param0,
        newownernode: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
            newownernode.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnSelectionChange<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, View>,
        Param1: ::windows::runtime::IntoParam<'a, Nodes>,
    >(
        &self,
        view: Param0,
        newnodes: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
            newnodes.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnContextMenuExecuted<'a, Param0: ::windows::runtime::IntoParam<'a, MenuItem>>(
        &self,
        menuitem: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            menuitem.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn OnToolbarButtonClicked(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnListUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, View>>(
        &self,
        view: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            view.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for _AppEvents {
    type Vtable = _AppEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3729181661,
        21493,
        17973,
        [175, 84, 79, 231, 30, 146, 61, 63],
    );
}
impl ::std::convert::From<_AppEvents> for ::windows::runtime::IUnknown {
    fn from(value: _AppEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_AppEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_AppEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<_AppEvents> for super::OleAutomation::IDispatch {
    fn from(value: _AppEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&_AppEvents> for super::OleAutomation::IDispatch {
    fn from(value: &_AppEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for _AppEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &_AppEvents {
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
pub struct _AppEvents_abi(
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
        application: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: ::windows::runtime::RawPtr,
        new: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: ::windows::runtime::RawPtr,
        snapin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: ::windows::runtime::RawPtr,
        snapin: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
        newownernode: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
        newnodes: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        menuitem: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct _Application(::windows::runtime::IUnknown);
impl _Application {
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
    pub unsafe fn Help(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn Quit(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<Document> {
        let mut result__: <Document as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Document>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        filename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            filename.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Frame(&self) -> ::windows::runtime::Result<Frame> {
        let mut result__: <Frame as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<Frame>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Visible(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Show(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Hide(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserControl(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserControl<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        usercontrol: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            usercontrol.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn VersionMajor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn VersionMinor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for _Application {
    type Vtable = _Application_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2746202572,
        46675,
        18241,
        [134, 171, 240, 71, 14, 193, 56, 76],
    );
}
impl ::std::convert::From<_Application> for ::windows::runtime::IUnknown {
    fn from(value: _Application) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_Application> for ::windows::runtime::IUnknown {
    fn from(value: &_Application) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _Application {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_Application {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<_Application> for super::OleAutomation::IDispatch {
    fn from(value: _Application) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&_Application> for super::OleAutomation::IDispatch {
    fn from(value: &_Application) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for _Application {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &_Application {
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
pub struct _Application_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        document: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        frame: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visible: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        usercontrol: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        usercontrol: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        versionmajor: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        versionminor: *mut i32,
    ) -> ::windows::runtime::HRESULT,
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
pub struct _ColumnSortOrder(pub i32);
pub const SortOrder_Ascending: _ColumnSortOrder = _ColumnSortOrder(0i32);
pub const SortOrder_Descending: _ColumnSortOrder = _ColumnSortOrder(1i32);
impl ::std::convert::From<i32> for _ColumnSortOrder {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _ColumnSortOrder {
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
pub struct _DocumentMode(pub i32);
pub const DocumentMode_Author: _DocumentMode = _DocumentMode(0i32);
pub const DocumentMode_User: _DocumentMode = _DocumentMode(1i32);
pub const DocumentMode_User_MDI: _DocumentMode = _DocumentMode(2i32);
pub const DocumentMode_User_SDI: _DocumentMode = _DocumentMode(3i32);
impl ::std::convert::From<i32> for _DocumentMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DocumentMode {
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
pub struct _EventConnector(::windows::runtime::IUnknown);
impl _EventConnector {
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
    pub unsafe fn ConnectTo<'a, Param0: ::windows::runtime::IntoParam<'a, _Application>>(
        &self,
        application: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            application.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for _EventConnector {
    type Vtable = _EventConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3233598768,
        56900,
        17704,
        [132, 3, 160, 90, 106, 28, 200, 234],
    );
}
impl ::std::convert::From<_EventConnector> for ::windows::runtime::IUnknown {
    fn from(value: _EventConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_EventConnector> for ::windows::runtime::IUnknown {
    fn from(value: &_EventConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _EventConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_EventConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<_EventConnector> for super::OleAutomation::IDispatch {
    fn from(value: _EventConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&_EventConnector> for super::OleAutomation::IDispatch {
    fn from(value: &_EventConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for _EventConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::OleAutomation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::OleAutomation::IDispatch> for &_EventConnector {
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
pub struct _EventConnector_abi(
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
        application: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
pub struct _ExportListOptions(pub i32);
pub const ExportListOptions_Default: _ExportListOptions = _ExportListOptions(0i32);
pub const ExportListOptions_Unicode: _ExportListOptions = _ExportListOptions(1i32);
pub const ExportListOptions_TabDelimited: _ExportListOptions = _ExportListOptions(2i32);
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = _ExportListOptions(4i32);
impl ::std::convert::From<i32> for _ExportListOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _ExportListOptions {
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
pub struct _ListViewMode(pub i32);
pub const ListMode_Small_Icons: _ListViewMode = _ListViewMode(0i32);
pub const ListMode_Large_Icons: _ListViewMode = _ListViewMode(1i32);
pub const ListMode_List: _ListViewMode = _ListViewMode(2i32);
pub const ListMode_Detail: _ListViewMode = _ListViewMode(3i32);
pub const ListMode_Filtered: _ListViewMode = _ListViewMode(4i32);
impl ::std::convert::From<i32> for _ListViewMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _ListViewMode {
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
pub struct _ViewOptions(pub i32);
pub const ViewOption_Default: _ViewOptions = _ViewOptions(0i32);
pub const ViewOption_ScopeTreeHidden: _ViewOptions = _ViewOptions(1i32);
pub const ViewOption_NoToolBars: _ViewOptions = _ViewOptions(2i32);
pub const ViewOption_NotPersistable: _ViewOptions = _ViewOptions(4i32);
pub const ViewOption_ActionPaneHidden: _ViewOptions = _ViewOptions(8i32);
impl ::std::convert::From<i32> for _ViewOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _ViewOptions {
    type Abi = Self;
    type DefaultType = Self;
}
