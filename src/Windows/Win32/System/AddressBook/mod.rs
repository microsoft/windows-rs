#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ADRENTRY").field("ulReserved1", &self.ulReserved1).field("cValues", &self.cValues).field("rgPropVals", &self.rgPropVals).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved1 == other.ulReserved1 && self.cValues == other.cValues && self.rgPropVals == other.rgPropVals
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for ADRENTRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRLIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ADRLIST").field("cEntries", &self.cEntries).field("aEntries", &self.aEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.aEntries == other.aEntries
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for ADRLIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRPARM {
    pub cbABContEntryID: u32,
    pub lpABContEntryID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
    pub ulHelpContext: u32,
    pub lpszHelpFileName: *mut i8,
    pub lpfnABSDI: ::core::option::Option<LPFNABSDI>,
    pub lpfnDismiss: ::core::option::Option<LPFNDISMISS>,
    pub lpvDismissContext: *mut ::core::ffi::c_void,
    pub lpszCaption: *mut i8,
    pub lpszNewEntryTitle: *mut i8,
    pub lpszDestWellsTitle: *mut i8,
    pub cDestFields: u32,
    pub nDestFieldFocus: u32,
    pub lppszDestTitles: *mut *mut i8,
    pub lpulDestComps: *mut u32,
    pub lpContRestriction: *mut SRestriction,
    pub lpHierRestriction: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ADRPARM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRPARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRPARM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ADRPARM")
            .field("cbABContEntryID", &self.cbABContEntryID)
            .field("lpABContEntryID", &self.lpABContEntryID)
            .field("ulFlags", &self.ulFlags)
            .field("lpReserved", &self.lpReserved)
            .field("ulHelpContext", &self.ulHelpContext)
            .field("lpszHelpFileName", &self.lpszHelpFileName)
            .field("lpvDismissContext", &self.lpvDismissContext)
            .field("lpszCaption", &self.lpszCaption)
            .field("lpszNewEntryTitle", &self.lpszNewEntryTitle)
            .field("lpszDestWellsTitle", &self.lpszDestWellsTitle)
            .field("cDestFields", &self.cDestFields)
            .field("nDestFieldFocus", &self.nDestFieldFocus)
            .field("lppszDestTitles", &self.lppszDestTitles)
            .field("lpulDestComps", &self.lpulDestComps)
            .field("lpContRestriction", &self.lpContRestriction)
            .field("lpHierRestriction", &self.lpHierRestriction)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRPARM {
    fn eq(&self, other: &Self) -> bool {
        self.cbABContEntryID == other.cbABContEntryID
            && self.lpABContEntryID == other.lpABContEntryID
            && self.ulFlags == other.ulFlags
            && self.lpReserved == other.lpReserved
            && self.ulHelpContext == other.ulHelpContext
            && self.lpszHelpFileName == other.lpszHelpFileName
            && self.lpfnABSDI.map(|f| f as usize) == other.lpfnABSDI.map(|f| f as usize)
            && self.lpfnDismiss.map(|f| f as usize) == other.lpfnDismiss.map(|f| f as usize)
            && self.lpvDismissContext == other.lpvDismissContext
            && self.lpszCaption == other.lpszCaption
            && self.lpszNewEntryTitle == other.lpszNewEntryTitle
            && self.lpszDestWellsTitle == other.lpszDestWellsTitle
            && self.cDestFields == other.cDestFields
            && self.nDestFieldFocus == other.nDestFieldFocus
            && self.lppszDestTitles == other.lppszDestTitles
            && self.lpulDestComps == other.lpulDestComps
            && self.lpContRestriction == other.lpContRestriction
            && self.lpHierRestriction == other.lpHierRestriction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRPARM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for ADRPARM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn BuildDisplayTable<'a, Param3: ::windows::core::IntoParam<'a, super::Com::IMalloc>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(
    lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>,
    lpallocatemore: ::core::option::Option<LPALLOCATEMORE>,
    lpfreebuffer: ::core::option::Option<LPFREEBUFFER>,
    lpmalloc: Param3,
    hinstance: Param4,
    cpages: u32,
    lppage: *mut DTPAGE,
    ulflags: u32,
    lpptable: *mut ::core::option::Option<IMAPITable>,
    lpptbldata: *mut ::core::option::Option<ITableData>,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildDisplayTable(lpallocatebuffer: ::windows::core::RawPtr, lpallocatemore: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, lpmalloc: ::windows::core::RawPtr, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut ::windows::core::RawPtr, lpptbldata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        BuildDisplayTable(
            ::core::mem::transmute(lpallocatebuffer),
            ::core::mem::transmute(lpallocatemore),
            ::core::mem::transmute(lpfreebuffer),
            lpmalloc.into_param().abi(),
            hinstance.into_param().abi(),
            ::core::mem::transmute(cpages),
            ::core::mem::transmute(lppage),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lpptable),
            ::core::mem::transmute(lpptbldata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type CALLERRELEASE = unsafe extern "system" fn(ulcallerdata: u32, lptbldata: ::windows::core::RawPtr, lpvue: ::windows::core::RawPtr);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: ::core::option::Option<PFNIDLE>, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: ::windows::core::RawPtr, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
        }
        ::core::mem::transmute(ChangeIdleRoutine(::core::mem::transmute(ftg), ::core::mem::transmute(lpfnidle), ::core::mem::transmute(lpvidleparam), ::core::mem::transmute(priidle), ::core::mem::transmute(csecidle), ::core::mem::transmute(iroidle), ::core::mem::transmute(ircidle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateIProp(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lpallocatemore: ::core::option::Option<LPALLOCATEMORE>, lpfreebuffer: ::core::option::Option<LPFREEBUFFER>, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut ::core::option::Option<IPropData>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIProp(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: ::windows::core::RawPtr, lpallocatemore: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(CreateIProp(::core::mem::transmute(lpinterface), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpallocatemore), ::core::mem::transmute(lpfreebuffer), ::core::mem::transmute(lpvreserved), ::core::mem::transmute(lpppropdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateTable(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lpallocatemore: ::core::option::Option<LPALLOCATEMORE>, lpfreebuffer: ::core::option::Option<LPFREEBUFFER>, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ::core::option::Option<ITableData>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTable(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: ::windows::core::RawPtr, lpallocatemore: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(CreateTable(
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpallocatebuffer),
            ::core::mem::transmute(lpallocatemore),
            ::core::mem::transmute(lpfreebuffer),
            ::core::mem::transmute(lpvreserved),
            ::core::mem::transmute(ultabletype),
            ::core::mem::transmute(ulproptagindexcolumn),
            ::core::mem::transmute(lpsproptagarraycolumns),
            ::core::mem::transmute(lpptabledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
impl DTBLBUTTON {}
impl ::core::default::Default for DTBLBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLBUTTON {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRControl", &self.ulPRControl).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulPRControl == other.ulPRControl
    }
}
impl ::core::cmp::Eq for DTBLBUTTON {}
unsafe impl ::windows::core::Abi for DTBLBUTTON {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
impl DTBLCHECKBOX {}
impl ::core::default::Default for DTBLCHECKBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLCHECKBOX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLCHECKBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRPropertyName", &self.ulPRPropertyName).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLCHECKBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulPRPropertyName == other.ulPRPropertyName
    }
}
impl ::core::cmp::Eq for DTBLCHECKBOX {}
unsafe impl ::windows::core::Abi for DTBLCHECKBOX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
impl DTBLCOMBOBOX {}
impl ::core::default::Default for DTBLCOMBOBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLCOMBOBOX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLCOMBOBOX").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPRPropertyName", &self.ulPRPropertyName).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLCOMBOBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszCharsAllowed == other.ulbLpszCharsAllowed && self.ulFlags == other.ulFlags && self.ulNumCharsAllowed == other.ulNumCharsAllowed && self.ulPRPropertyName == other.ulPRPropertyName && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLCOMBOBOX {}
unsafe impl ::windows::core::Abi for DTBLCOMBOBOX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl DTBLDDLBX {}
impl ::core::default::Default for DTBLDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLDDLBX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLDDLBX").field("ulFlags", &self.ulFlags).field("ulPRDisplayProperty", &self.ulPRDisplayProperty).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLDDLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulPRDisplayProperty == other.ulPRDisplayProperty && self.ulPRSetProperty == other.ulPRSetProperty && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLDDLBX {}
unsafe impl ::windows::core::Abi for DTBLDDLBX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
impl DTBLEDIT {}
impl ::core::default::Default for DTBLEDIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLEDIT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLEDIT").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPropTag", &self.ulPropTag).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLEDIT {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszCharsAllowed == other.ulbLpszCharsAllowed && self.ulFlags == other.ulFlags && self.ulNumCharsAllowed == other.ulNumCharsAllowed && self.ulPropTag == other.ulPropTag
    }
}
impl ::core::cmp::Eq for DTBLEDIT {}
unsafe impl ::windows::core::Abi for DTBLEDIT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
impl DTBLGROUPBOX {}
impl ::core::default::Default for DTBLGROUPBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLGROUPBOX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLGROUPBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLGROUPBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DTBLGROUPBOX {}
unsafe impl ::windows::core::Abi for DTBLGROUPBOX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
impl DTBLLABEL {}
impl ::core::default::Default for DTBLLABEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLLABEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLLABEL").field("ulbLpszLabelName", &self.ulbLpszLabelName).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLLABEL {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabelName == other.ulbLpszLabelName && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DTBLLABEL {}
unsafe impl ::windows::core::Abi for DTBLLABEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl DTBLLBX {}
impl ::core::default::Default for DTBLLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLLBX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLLBX").field("ulFlags", &self.ulFlags).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulPRSetProperty == other.ulPRSetProperty && self.ulPRTableName == other.ulPRTableName
    }
}
impl ::core::cmp::Eq for DTBLLBX {}
unsafe impl ::windows::core::Abi for DTBLLBX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl DTBLMVDDLBX {}
impl ::core::default::Default for DTBLMVDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLMVDDLBX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLMVDDLBX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLMVDDLBX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulMVPropTag == other.ulMVPropTag
    }
}
impl ::core::cmp::Eq for DTBLMVDDLBX {}
unsafe impl ::windows::core::Abi for DTBLMVDDLBX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl DTBLMVLISTBOX {}
impl ::core::default::Default for DTBLMVLISTBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLMVLISTBOX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLMVLISTBOX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLMVLISTBOX {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulMVPropTag == other.ulMVPropTag
    }
}
impl ::core::cmp::Eq for DTBLMVLISTBOX {}
unsafe impl ::windows::core::Abi for DTBLMVLISTBOX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
impl DTBLPAGE {}
impl ::core::default::Default for DTBLPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLPAGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLPAGE").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulbLpszComponent", &self.ulbLpszComponent).field("ulContext", &self.ulContext).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLPAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulbLpszComponent == other.ulbLpszComponent && self.ulContext == other.ulContext
    }
}
impl ::core::cmp::Eq for DTBLPAGE {}
unsafe impl ::windows::core::Abi for DTBLPAGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
}
impl DTBLRADIOBUTTON {}
impl ::core::default::Default for DTBLRADIOBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DTBLRADIOBUTTON {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DTBLRADIOBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulcButtons", &self.ulcButtons).field("ulPropTag", &self.ulPropTag).field("lReturnValue", &self.lReturnValue).finish()
    }
}
impl ::core::cmp::PartialEq for DTBLRADIOBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.ulbLpszLabel == other.ulbLpszLabel && self.ulFlags == other.ulFlags && self.ulcButtons == other.ulcButtons && self.ulPropTag == other.ulPropTag && self.lReturnValue == other.lReturnValue
    }
}
impl ::core::cmp::Eq for DTBLRADIOBUTTON {}
unsafe impl ::windows::core::Abi for DTBLRADIOBUTTON {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTCTL {
    pub ulCtlType: u32,
    pub ulCtlFlags: u32,
    pub lpbNotif: *mut u8,
    pub cbNotif: u32,
    pub lpszFilter: *mut i8,
    pub ulItemID: u32,
    pub ctl: DTCTL_0,
}
impl DTCTL {}
impl ::core::default::Default for DTCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTCTL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DTCTL {}
unsafe impl ::windows::core::Abi for DTCTL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union DTCTL_0 {
    pub lpv: *mut ::core::ffi::c_void,
    pub lplabel: *mut DTBLLABEL,
    pub lpedit: *mut DTBLEDIT,
    pub lplbx: *mut DTBLLBX,
    pub lpcombobox: *mut DTBLCOMBOBOX,
    pub lpddlbx: *mut DTBLDDLBX,
    pub lpcheckbox: *mut DTBLCHECKBOX,
    pub lpgroupbox: *mut DTBLGROUPBOX,
    pub lpbutton: *mut DTBLBUTTON,
    pub lpradiobutton: *mut DTBLRADIOBUTTON,
    pub lpmvlbx: *mut DTBLMVLISTBOX,
    pub lpmvddlbx: *mut DTBLMVDDLBX,
    pub lppage: *mut DTBLPAGE,
}
impl DTCTL_0 {}
impl ::core::default::Default for DTCTL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTCTL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DTCTL_0 {}
unsafe impl ::windows::core::Abi for DTCTL_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DTPAGE {
    pub cctl: u32,
    pub lpszResourceName: *mut i8,
    pub Anonymous: DTPAGE_0,
    pub lpctl: *mut DTCTL,
}
impl DTPAGE {}
impl ::core::default::Default for DTPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTPAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DTPAGE {}
unsafe impl ::windows::core::Abi for DTPAGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union DTPAGE_0 {
    pub lpszComponent: *mut i8,
    pub ulItemID: u32,
}
impl DTPAGE_0 {}
impl ::core::default::Default for DTPAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DTPAGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DTPAGE_0 {}
unsafe impl ::windows::core::Abi for DTPAGE_0 {
    type Abi = Self;
}
#[inline]
pub unsafe fn DeinitMapiUtil() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeinitMapiUtil();
        }
        ::core::mem::transmute(DeinitMapiUtil())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(DeregisterIdleRoutine(::core::mem::transmute(ftg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
impl ENTRYID {}
impl ::core::default::Default for ENTRYID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENTRYID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENTRYID").field("abFlags", &self.abFlags).field("ab", &self.ab).finish()
    }
}
impl ::core::cmp::PartialEq for ENTRYID {
    fn eq(&self, other: &Self) -> bool {
        self.abFlags == other.abFlags && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for ENTRYID {}
unsafe impl ::windows::core::Abi for ENTRYID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
impl ERROR_NOTIFICATION {}
impl ::core::default::Default for ERROR_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ERROR_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ERROR_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("scode", &self.scode).field("ulFlags", &self.ulFlags).field("lpMAPIError", &self.lpMAPIError).finish()
    }
}
impl ::core::cmp::PartialEq for ERROR_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.scode == other.scode && self.ulFlags == other.ulFlags && self.lpMAPIError == other.lpMAPIError
    }
}
impl ::core::cmp::Eq for ERROR_NOTIFICATION {}
unsafe impl ::windows::core::Abi for ERROR_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
impl EXTENDED_NOTIFICATION {}
impl ::core::default::Default for EXTENDED_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EXTENDED_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EXTENDED_NOTIFICATION").field("ulEvent", &self.ulEvent).field("cb", &self.cb).field("pbEventParameters", &self.pbEventParameters).finish()
    }
}
impl ::core::cmp::PartialEq for EXTENDED_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.cb == other.cb && self.pbEventParameters == other.pbEventParameters
    }
}
impl ::core::cmp::Eq for EXTENDED_NOTIFICATION {}
unsafe impl ::windows::core::Abi for EXTENDED_NOTIFICATION {
    type Abi = Self;
}
pub const E_IMAPI_BURN_VERIFICATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600697i32 as _);
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599672i32 as _);
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599678i32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599674i32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599675i32 as _);
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599673i32 as _);
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599677i32 as _);
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599676i32 as _);
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599680i32 as _);
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599679i32 as _);
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599164i32 as _);
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599154i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599162i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599166i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599161i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599165i32 as _);
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599159i32 as _);
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599158i32 as _);
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599152i32 as _);
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599153i32 as _);
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599155i32 as _);
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599168i32 as _);
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599167i32 as _);
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599409i32 as _);
pub const E_IMAPI_DF2TAO_INVALID_ISRC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599413i32 as _);
pub const E_IMAPI_DF2TAO_INVALID_MCN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599412i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599418i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599422i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599417i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599421i32 as _);
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599415i32 as _);
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599414i32 as _);
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599420i32 as _);
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599410i32 as _);
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599411i32 as _);
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599419i32 as _);
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599416i32 as _);
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599424i32 as _);
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599423i32 as _);
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598389i32 as _);
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340222i32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340219i32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340216i32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340220i32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598391i32 as _);
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340221i32 as _);
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340223i32 as _);
pub const E_IMAPI_ERASE_RECORDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340224i32 as _);
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598390i32 as _);
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340218i32 as _);
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340217i32 as _);
pub const E_IMAPI_LOSS_OF_STREAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599936i32 as _);
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339963i32 as _);
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339968i32 as _);
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339965i32 as _);
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339966i32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339967i32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339962i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339964i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339961i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339959i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339958i32 as _);
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600175i32 as _);
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600179i32 as _);
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600178i32 as _);
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600181i32 as _);
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600180i32 as _);
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600184i32 as _);
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599937i32 as _);
pub const E_IMAPI_RECORDER_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600176i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600187i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600185i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600186i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600189i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600174i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600190i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600177i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600188i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600183i32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600182i32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600191i32 as _);
pub const E_IMAPI_RECORDER_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600701i32 as _);
pub const E_IMAPI_REQUEST_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600702i32 as _);
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599935i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableIdleRoutine<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ftg: *mut ::core::ffi::c_void, fenable: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableIdleRoutine(ftg: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL);
        }
        ::core::mem::transmute(EnableIdleRoutine(::core::mem::transmute(ftg), fenable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FACILITY_IMAPI2: u32 = 170u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FEqualNames(::core::mem::transmute(lpname1), ::core::mem::transmute(lpname2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
impl FLATENTRY {}
impl ::core::default::Default for FLATENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FLATENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLATENTRY").field("cb", &self.cb).field("abEntry", &self.abEntry).finish()
    }
}
impl ::core::cmp::PartialEq for FLATENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.abEntry == other.abEntry
    }
}
impl ::core::cmp::Eq for FLATENTRY {}
unsafe impl ::windows::core::Abi for FLATENTRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
impl FLATENTRYLIST {}
impl ::core::default::Default for FLATENTRYLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FLATENTRYLIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLATENTRYLIST").field("cEntries", &self.cEntries).field("cbEntries", &self.cbEntries).field("abEntries", &self.abEntries).finish()
    }
}
impl ::core::cmp::PartialEq for FLATENTRYLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.cbEntries == other.cbEntries && self.abEntries == other.abEntries
    }
}
impl ::core::cmp::Eq for FLATENTRYLIST {}
unsafe impl ::windows::core::Abi for FLATENTRYLIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
impl FLATMTSIDLIST {}
impl ::core::default::Default for FLATMTSIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FLATMTSIDLIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLATMTSIDLIST").field("cMTSIDs", &self.cMTSIDs).field("cbMTSIDs", &self.cbMTSIDs).field("abMTSIDs", &self.abMTSIDs).finish()
    }
}
impl ::core::cmp::PartialEq for FLATMTSIDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cMTSIDs == other.cMTSIDs && self.cbMTSIDs == other.cbMTSIDs && self.abMTSIDs == other.abMTSIDs
    }
}
impl ::core::cmp::Eq for FLATMTSIDLIST {}
unsafe impl ::windows::core::Abi for FLATMTSIDLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type FNIDLE = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FPropCompareProp(::core::mem::transmute(lpspropvalue1), ::core::mem::transmute(ulrelop), ::core::mem::transmute(lpspropvalue2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FPropContainsProp(::core::mem::transmute(lpspropvaluedst), ::core::mem::transmute(lpspropvaluesrc), ::core::mem::transmute(ulfuzzylevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FPropExists<'a, Param0: ::windows::core::IntoParam<'a, IMAPIProp>>(lpmapiprop: Param0, ulproptag: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FPropExists(lpmapiprop: ::windows::core::RawPtr, ulproptag: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FPropExists(lpmapiprop.into_param().abi(), ::core::mem::transmute(ulproptag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FreePadrlist(lpadrlist: *mut ADRLIST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreePadrlist(lpadrlist: *mut ADRLIST);
        }
        ::core::mem::transmute(FreePadrlist(::core::mem::transmute(lpadrlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FreeProws(lprows: *mut SRowSet) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeProws(lprows: *mut SRowSet);
        }
        ::core::mem::transmute(FreeProws(::core::mem::transmute(lprows)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtAddFt<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ftaddend1: Param0, ftaddend2: Param1) -> super::super::Foundation::FILETIME {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
        }
        ::core::mem::transmute(FtAddFt(ftaddend1.into_param().abi(), ftaddend2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtMulDw<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ftmultiplier: u32, ftmultiplicand: Param1) -> super::super::Foundation::FILETIME {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
        }
        ::core::mem::transmute(FtMulDw(::core::mem::transmute(ftmultiplier), ftmultiplicand.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME;
        }
        ::core::mem::transmute(FtMulDwDw(::core::mem::transmute(ftmultiplicand), ::core::mem::transmute(ftmultiplier)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtNegFt<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ft: Param0) -> super::super::Foundation::FILETIME {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
        }
        ::core::mem::transmute(FtNegFt(ft.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtSubFt<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ftminuend: Param0, ftsubtrahend: Param1) -> super::super::Foundation::FILETIME {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
        }
        ::core::mem::transmute(FtSubFt(ftminuend.into_param().abi(), ftsubtrahend.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtgRegisterIdleRoutine(lpfnidle: ::core::option::Option<PFNIDLE>, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtgRegisterIdleRoutine(lpfnidle: ::windows::core::RawPtr, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FtgRegisterIdleRoutine(::core::mem::transmute(lpfnidle), ::core::mem::transmute(lpvidleparam), ::core::mem::transmute(priidle), ::core::mem::transmute(csecidle), ::core::mem::transmute(iroidle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Gender(pub i32);
pub const genderUnspecified: Gender = Gender(0i32);
pub const genderFemale: Gender = Gender(1i32);
pub const genderMale: Gender = Gender(2i32);
impl ::core::convert::From<i32> for Gender {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Gender {
    type Abi = Self;
}
#[inline]
pub unsafe fn HrAddColumns<'a, Param0: ::windows::core::IntoParam<'a, IMAPITable>>(lptbl: Param0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lpfreebuffer: ::core::option::Option<LPFREEBUFFER>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrAddColumns(lptbl: ::windows::core::RawPtr, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HrAddColumns(lptbl.into_param().abi(), ::core::mem::transmute(lpproptagcolumnsnew), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HrAddColumnsEx<'a, Param0: ::windows::core::IntoParam<'a, IMAPITable>>(lptbl: Param0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lpfreebuffer: ::core::option::Option<LPFREEBUFFER>, lpfnfiltercolumns: isize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrAddColumnsEx(lptbl: ::windows::core::RawPtr, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, lpfnfiltercolumns: isize) -> ::windows::core::HRESULT;
        }
        HrAddColumnsEx(lptbl.into_param().abi(), ::core::mem::transmute(lpproptagcolumnsnew), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer), ::core::mem::transmute(lpfnfiltercolumns)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrAllocAdviseSink(lpfncallback: ::core::option::Option<LPNOTIFCALLBACK>, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut ::core::option::Option<IMAPIAdviseSink>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrAllocAdviseSink(lpfncallback: ::windows::core::RawPtr, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HrAllocAdviseSink(::core::mem::transmute(lpfncallback), ::core::mem::transmute(lpvcontext), ::core::mem::transmute(lppadvisesink)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HrDispatchNotifications(ulflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrDispatchNotifications(ulflags: u32) -> ::windows::core::HRESULT;
        }
        HrDispatchNotifications(::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrGetOneProp<'a, Param0: ::windows::core::IntoParam<'a, IMAPIProp>>(lpmapiprop: Param0, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrGetOneProp(lpmapiprop: ::windows::core::RawPtr, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows::core::HRESULT;
        }
        HrGetOneProp(lpmapiprop.into_param().abi(), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lppprop)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn HrIStorageFromStream<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(lpunkin: Param0, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppstorageout: *mut ::core::option::Option<super::Com::StructuredStorage::IStorage>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrIStorageFromStream(lpunkin: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppstorageout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HrIStorageFromStream(lpunkin.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppstorageout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrQueryAllRows<'a, Param0: ::windows::core::IntoParam<'a, IMAPITable>>(lptable: Param0, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrQueryAllRows(lptable: ::windows::core::RawPtr, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT;
        }
        HrQueryAllRows(lptable.into_param().abi(), ::core::mem::transmute(lpproptags), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpsortorderset), ::core::mem::transmute(crowsmax), ::core::mem::transmute(lpprows)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrSetOneProp<'a, Param0: ::windows::core::IntoParam<'a, IMAPIProp>>(lpmapiprop: Param0, lpprop: *mut SPropValue) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrSetOneProp(lpmapiprop: ::windows::core::RawPtr, lpprop: *mut SPropValue) -> ::windows::core::HRESULT;
        }
        HrSetOneProp(lpmapiprop.into_param().abi(), ::core::mem::transmute(lpprop)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HrThisThreadAdviseSink<'a, Param0: ::windows::core::IntoParam<'a, IMAPIAdviseSink>>(lpadvisesink: Param0) -> ::windows::core::Result<IMAPIAdviseSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HrThisThreadAdviseSink(lpadvisesink: ::windows::core::RawPtr, lppadvisesink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMAPIAdviseSink as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        HrThisThreadAdviseSink(lpadvisesink.into_param().abi(), &mut result__).from_abi::<IMAPIAdviseSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IABContainer(pub ::windows::core::IUnknown);
impl IABContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ::core::mem::transmute(ulsearchflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp> {
        let mut result__: <IMAPIProp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(ulcreateflags), &mut result__).from_abi::<IMAPIProp>(result__)
    }
    pub unsafe fn CopyEntries<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: Param2, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpentries), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpentries), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist> {
        let mut result__: <_flaglist as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpadrlist), &mut result__).from_abi::<_flaglist>(result__)
    }
}
unsafe impl ::windows::core::Interface for IABContainer {
    type Vtable = IABContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IABContainer> for ::windows::core::IUnknown {
    fn from(value: IABContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IABContainer> for ::windows::core::IUnknown {
    fn from(value: &IABContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IABContainer> for IMAPIContainer {
    fn from(value: IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IABContainer> for IMAPIContainer {
    fn from(value: &IABContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for &IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IABContainer> for IMAPIProp {
    fn from(value: IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IABContainer> for IMAPIProp {
    fn from(value: &IABContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IABContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IABContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAddrBook(pub ::windows::core::IUnknown);
impl IAddrBook {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid1), ::core::mem::transmute(lpentryid1), ::core::mem::transmute(cbentryid2), ::core::mem::transmute(lpentryid2), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulresult)).ok()
    }
    pub unsafe fn Advise<'a, Param3: ::windows::core::IntoParam<'a, IMAPIAdviseSink>>(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: Param3, lpulconnection: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(uleventmask), lpadvisesink.into_param().abi(), ::core::mem::transmute(lpulconnection)).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulconnection)).ok()
    }
    pub unsafe fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpszname), ::core::mem::transmute(lpszadrtype), ::core::mem::transmute(lpszaddress), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(uluiparam),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(cbeidcontainer),
            ::core::mem::transmute(lpeidcontainer),
            ::core::mem::transmute(cbeidnewentrytpl),
            ::core::mem::transmute(lpeidnewentrytpl),
            ::core::mem::transmute(lpcbeidnewentry),
            ::core::mem::transmute(lppeidnewentry),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpsznewentrytitle), ::core::mem::transmute(lpadrlist)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpuluiparam), ::core::mem::transmute(lpadrparms), ::core::mem::transmute(lppadrlist)).ok()
    }
    pub unsafe fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: ::core::option::Option<LPFNDISMISS>, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: ::core::option::Option<LPFNBUTTON>, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(lpuluiparam),
            ::core::mem::transmute(lpfndismiss),
            ::core::mem::transmute(lpvdismisscontext),
            ::core::mem::transmute(cbentryid),
            ::core::mem::transmute(lpentryid),
            ::core::mem::transmute(lpfbuttoncallback),
            ::core::mem::transmute(lpvbuttoncontext),
            ::core::mem::transmute(lpszbuttontext),
            ::core::mem::transmute(ulflags),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), ::core::mem::transmute(ulflags), ::core::mem::transmute(lprecip)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpszadrtype), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppoptions)).ok()
    }
    pub unsafe fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid)).ok()
    }
    pub unsafe fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppsearchpath)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpsearchpath)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lpreciplist)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAddrBook {
    type Vtable = IAddrBook_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IAddrBook> for ::windows::core::IUnknown {
    fn from(value: IAddrBook) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAddrBook> for ::windows::core::IUnknown {
    fn from(value: &IAddrBook) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrBook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAddrBook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAddrBook> for IMAPIProp {
    fn from(value: IAddrBook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrBook> for IMAPIProp {
    fn from(value: &IAddrBook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IAddrBook {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IAddrBook {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrBook_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpuluiparam: *mut u32, lpadrparms: *mut ::core::mem::ManuallyDrop<ADRPARM>, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpuluiparam: *mut usize, lpfndismiss: ::windows::core::RawPtr, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: ::windows::core::RawPtr, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAttach(pub ::windows::core::IUnknown);
impl IAttach {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAttach {
    type Vtable = IAttach_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IAttach> for ::windows::core::IUnknown {
    fn from(value: IAttach) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAttach> for ::windows::core::IUnknown {
    fn from(value: &IAttach) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAttach {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAttach {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAttach> for IMAPIProp {
    fn from(value: IAttach) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAttach> for IMAPIProp {
    fn from(value: &IAttach) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IAttach {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IAttach {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttach_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDistList(pub ::windows::core::IUnknown);
impl IDistList {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ::core::mem::transmute(ulsearchflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp> {
        let mut result__: <IMAPIProp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(ulcreateflags), &mut result__).from_abi::<IMAPIProp>(result__)
    }
    pub unsafe fn CopyEntries<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: Param2, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpentries), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpentries), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist> {
        let mut result__: <_flaglist as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpadrlist), &mut result__).from_abi::<_flaglist>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDistList {
    type Vtable = IDistList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IDistList> for ::windows::core::IUnknown {
    fn from(value: IDistList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDistList> for ::windows::core::IUnknown {
    fn from(value: &IDistList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDistList> for IMAPIContainer {
    fn from(value: IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDistList> for IMAPIContainer {
    fn from(value: &IDistList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for &IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDistList> for IMAPIProp {
    fn from(value: IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDistList> for IMAPIProp {
    fn from(value: &IDistList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IDistList {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDistList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIAdviseSink(pub ::windows::core::IUnknown);
impl IMAPIAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnotif), ::core::mem::transmute(lpnotifications)))
    }
}
unsafe impl ::windows::core::Interface for IMAPIAdviseSink {
    type Vtable = IMAPIAdviseSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IMAPIAdviseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IMAPIAdviseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIContainer(pub ::windows::core::IUnknown);
impl IMAPIContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ::core::mem::transmute(ulsearchflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIContainer {
    type Vtable = IMAPIContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIContainer> for ::windows::core::IUnknown {
    fn from(value: IMAPIContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIContainer> for ::windows::core::IUnknown {
    fn from(value: &IMAPIContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMAPIContainer> for IMAPIProp {
    fn from(value: IMAPIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIContainer> for IMAPIProp {
    fn from(value: &IMAPIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMAPIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMAPIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIControl(pub ::windows::core::IUnknown);
impl IMAPIControl {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR> {
        let mut result__: <*mut MAPIERROR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), &mut result__).from_abi::<*mut MAPIERROR>(result__)
    }
    pub unsafe fn Activate(&self, ulflags: u32, uluiparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(uluiparam)).ok()
    }
    pub unsafe fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulstate)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIControl {
    type Vtable = IMAPIControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIControl> for ::windows::core::IUnknown {
    fn from(value: IMAPIControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIControl> for ::windows::core::IUnknown {
    fn from(value: &IMAPIControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, uluiparam: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIFolder(pub ::windows::core::IUnknown);
impl IMAPIFolder {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ::core::mem::transmute(ulsearchflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateMessage(&self, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::core::option::Option<IMessage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmessage)).ok()
    }
    pub unsafe fn CopyMessages<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: Param4, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpmsglist), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestfolder), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn DeleteMessages<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: Param2, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpmsglist), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IMAPIFolder> {
        let mut result__: <IMAPIFolder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulfoldertype), ::core::mem::transmute(lpszfoldername), ::core::mem::transmute(lpszfoldercomment), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPIFolder>(result__)
    }
    pub unsafe fn CopyFolder<'a, Param6: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: Param6, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(cbentryid),
            ::core::mem::transmute(lpentryid),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestfolder),
            ::core::mem::transmute(lpsznewfoldername),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(ulflags),
        )
        .ok()
    }
    pub unsafe fn DeleteFolder<'a, Param3: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: Param3, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn SetReadFlags<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: Param2, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpmsglist), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(ulflags), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(ulnewstatus), ::core::mem::transmute(ulnewstatusmask), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpsortcriteria), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn EmptyFolder<'a, Param1: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, uluiparam: usize, lpprogress: Param1, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIFolder {
    type Vtable = IMAPIFolder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIFolder> for ::windows::core::IUnknown {
    fn from(value: IMAPIFolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIFolder> for ::windows::core::IUnknown {
    fn from(value: &IMAPIFolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMAPIFolder> for IMAPIContainer {
    fn from(value: IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIFolder> for IMAPIContainer {
    fn from(value: &IMAPIFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIContainer> for &IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIContainer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMAPIFolder> for IMAPIProp {
    fn from(value: IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIFolder> for IMAPIProp {
    fn from(value: &IMAPIFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMAPIFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIFolder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIProgress(pub ::windows::core::IUnknown);
impl IMAPIProgress {
    pub unsafe fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulvalue), ::core::mem::transmute(ulcount), ::core::mem::transmute(ultotal)).ok()
    }
    pub unsafe fn GetFlags(&self, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulflags)).ok()
    }
    pub unsafe fn GetMax(&self, lpulmax: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulmax)).ok()
    }
    pub unsafe fn GetMin(&self, lpulmin: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulmin)).ok()
    }
    pub unsafe fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulmin), ::core::mem::transmute(lpulmax), ::core::mem::transmute(lpulflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIProgress {
    type Vtable = IMAPIProgress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIProgress> for ::windows::core::IUnknown {
    fn from(value: IMAPIProgress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIProgress> for ::windows::core::IUnknown {
    fn from(value: &IMAPIProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulmin: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIProp(pub ::windows::core::IUnknown);
impl IMAPIProp {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIProp {
    type Vtable = IMAPIProp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIProp> for ::windows::core::IUnknown {
    fn from(value: IMAPIProp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIProp> for ::windows::core::IUnknown {
    fn from(value: &IMAPIProp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIProp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIProp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIProp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPIStatus(pub ::windows::core::IUnknown);
impl IMAPIStatus {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn ValidateState(&self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn SettingsDialog(&self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpoldpass), ::core::mem::transmute(lpnewpass), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn FlushQueues(&self, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uluiparam), ::core::mem::transmute(cbtargettransport), ::core::mem::transmute(lptargettransport), ::core::mem::transmute(ulflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPIStatus {
    type Vtable = IMAPIStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPIStatus> for ::windows::core::IUnknown {
    fn from(value: IMAPIStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPIStatus> for ::windows::core::IUnknown {
    fn from(value: &IMAPIStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMAPIStatus> for IMAPIProp {
    fn from(value: IMAPIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIStatus> for IMAPIProp {
    fn from(value: &IMAPIStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMAPIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMAPIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMAPITable(pub ::windows::core::IUnknown);
impl IMAPITable {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn Advise<'a, Param1: ::windows::core::IntoParam<'a, IMAPIAdviseSink>>(&self, uleventmask: u32, lpadvisesink: Param1, lpulconnection: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uleventmask), lpadvisesink.into_param().abi(), ::core::mem::transmute(lpulconnection)).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulconnection)).ok()
    }
    pub unsafe fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpultablestatus), ::core::mem::transmute(lpultabletype)).ok()
    }
    pub unsafe fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpproptagarray)).ok()
    }
    pub unsafe fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulcount)).ok()
    }
    pub unsafe fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(bkorigin), ::core::mem::transmute(lrowcount), ::core::mem::transmute(lplrowssought)).ok()
    }
    pub unsafe fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulnumerator), ::core::mem::transmute(uldenominator)).ok()
    }
    pub unsafe fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulrow), ::core::mem::transmute(lpulnumerator), ::core::mem::transmute(lpuldenominator)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(bkorigin), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn CreateBookmark(&self, lpbkposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbkposition)).ok()
    }
    pub unsafe fn FreeBookmark(&self, bkposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bkposition)).ok()
    }
    pub unsafe fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpsortcriteria), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppsortcriteria)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowcount), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprows)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbinstancekey), ::core::mem::transmute(pbinstancekey), ::core::mem::transmute(ulrowcount), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpprows), ::core::mem::transmute(lpulmorerows)).ok()
    }
    pub unsafe fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbinstancekey), ::core::mem::transmute(pbinstancekey), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulrowcount)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(ultimeout), ::core::mem::transmute(lpultablestatus)).ok()
    }
    pub unsafe fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(cbinstancekey), ::core::mem::transmute(lpbinstancekey), ::core::mem::transmute(lpcbcollapsestate), ::core::mem::transmute(lppbcollapsestate)).ok()
    }
    pub unsafe fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(cbcollapsestate), ::core::mem::transmute(pbcollapsestate), ::core::mem::transmute(lpbklocation)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMAPITable {
    type Vtable = IMAPITable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMAPITable> for ::windows::core::IUnknown {
    fn from(value: IMAPITable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMAPITable> for ::windows::core::IUnknown {
    fn from(value: &IMAPITable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMAPITable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMAPITable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPITable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulnumerator: u32, uldenominator: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpbkposition: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bkposition: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::HRESULT,
);
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555294i32 as _);
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555318i32 as _);
pub const IMAPI_E_BOOT_IMAGE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555320i32 as _);
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555319i32 as _);
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555350i32 as _);
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555352i32 as _);
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555351i32 as _);
pub const IMAPI_E_DATA_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555342i32 as _);
pub const IMAPI_E_DIRECTORY_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555349i32 as _);
pub const IMAPI_E_DIR_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555382i32 as _);
pub const IMAPI_E_DIR_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555366i32 as _);
pub const IMAPI_E_DISC_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555304i32 as _);
pub const IMAPI_E_DUP_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555374i32 as _);
pub const IMAPI_E_EMPTY_DISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555312i32 as _);
pub const IMAPI_E_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555367i32 as _);
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555293i32 as _);
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555308i32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555386i32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555310i32 as _);
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555309i32 as _);
pub const IMAPI_E_FSI_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555392i32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555136i32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555133i32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555134i32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555135i32 as _);
pub const IMAPI_E_IMAGE_SIZE_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555360i32 as _);
pub const IMAPI_E_IMAGE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555359i32 as _);
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555303i32 as _);
pub const IMAPI_E_IMPORT_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555305i32 as _);
pub const IMAPI_E_IMPORT_SEEK_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555306i32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555298i32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555307i32 as _);
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555301i32 as _);
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555341i32 as _);
pub const IMAPI_E_INVALID_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555387i32 as _);
pub const IMAPI_E_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555391i32 as _);
pub const IMAPI_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555376i32 as _);
pub const IMAPI_E_INVALID_VOLUME_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555388i32 as _);
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555328i32 as _);
pub const IMAPI_E_ISO9660_LEVELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555343i32 as _);
pub const IMAPI_E_ITEM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555368i32 as _);
pub const IMAPI_E_MULTISESSION_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555299i32 as _);
pub const IMAPI_E_NOT_DIR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555383i32 as _);
pub const IMAPI_E_NOT_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555384i32 as _);
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555381i32 as _);
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555300i32 as _);
pub const IMAPI_E_NO_OUTPUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555389i32 as _);
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555311i32 as _);
pub const IMAPI_E_NO_UNIQUE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555373i32 as _);
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555296i32 as _);
pub const IMAPI_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555390i32 as _);
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555375i32 as _);
pub const IMAPI_E_STASHFILE_MOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555326i32 as _);
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555336i32 as _);
pub const IMAPI_E_STASHFILE_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555333i32 as _);
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555335i32 as _);
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555334i32 as _);
pub const IMAPI_E_TOO_MANY_DIRS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555344i32 as _);
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555302i32 as _);
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555295i32 as _);
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555327i32 as _);
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11186527i32 as _);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMailUser(pub ::windows::core::IUnknown);
impl IMailUser {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMailUser {
    type Vtable = IMailUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMailUser> for ::windows::core::IUnknown {
    fn from(value: IMailUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMailUser> for ::windows::core::IUnknown {
    fn from(value: &IMailUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMailUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMailUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMailUser> for IMAPIProp {
    fn from(value: IMailUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMailUser> for IMAPIProp {
    fn from(value: &IMailUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMailUser {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMailUser {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMailUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMessage(pub ::windows::core::IUnknown);
impl IMessage {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetAttachmentTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IAttach> {
        let mut result__: <IAttach as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulattachmentnum), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IAttach>(result__)
    }
    pub unsafe fn CreateAttach(&self, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::core::option::Option<IAttach>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulattachmentnum), ::core::mem::transmute(lppattach)).ok()
    }
    pub unsafe fn DeleteAttach<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ulattachmentnum: u32, uluiparam: usize, lpprogress: Param2, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulattachmentnum), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn GetRecipientTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpmods)).ok()
    }
    pub unsafe fn SubmitMessage(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn SetReadFlag(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMessage {
    type Vtable = IMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMessage> for ::windows::core::IUnknown {
    fn from(value: IMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMessage> for ::windows::core::IUnknown {
    fn from(value: &IMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMessage> for IMAPIProp {
    fn from(value: IMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessage> for IMAPIProp {
    fn from(value: &IMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulattachmentnum: u32, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMsgStore(pub ::windows::core::IUnknown);
impl IMsgStore {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn Advise<'a, Param3: ::windows::core::IntoParam<'a, IMAPIAdviseSink>>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: Param3) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(uleventmask), lpadvisesink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulconnection)).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid1), ::core::mem::transmute(lpentryid1), ::core::mem::transmute(cbentryid2), ::core::mem::transmute(lpentryid2), ::core::mem::transmute(ulflags), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn SetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpszmessageclass), ::core::mem::transmute(ulflags), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid)).ok()
    }
    pub unsafe fn GetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpszmessageclass), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid), ::core::mem::transmute(lppszexplicitclass)).ok()
    }
    pub unsafe fn GetReceiveFolderTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn StoreLogoff(&self, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpulflags)).ok()
    }
    pub unsafe fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid), ::core::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn GetOutgoingQueue(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn SetLockState<'a, Param0: ::windows::core::IntoParam<'a, IMessage>>(&self, lpmessage: Param0, ullockstate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), lpmessage.into_param().abi(), ::core::mem::transmute(ullockstate)).ok()
    }
    pub unsafe fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(cbentryid), ::core::mem::transmute(lpentryid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpnotification)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMsgStore {
    type Vtable = IMsgStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IMsgStore> for ::windows::core::IUnknown {
    fn from(value: IMsgStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMsgStore> for ::windows::core::IUnknown {
    fn from(value: &IMsgStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMsgStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMsgStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMsgStore> for IMAPIProp {
    fn from(value: IMsgStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMsgStore> for IMAPIProp {
    fn from(value: &IMsgStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IMsgStore {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IMsgStore {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsgStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpulflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpmessage: ::windows::core::RawPtr, ullockstate: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpnotification: *const NOTIFICATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProfSect(pub ::windows::core::IUnknown);
impl IProfSect {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IProfSect {
    type Vtable = IProfSect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IProfSect> for ::windows::core::IUnknown {
    fn from(value: IProfSect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProfSect> for ::windows::core::IUnknown {
    fn from(value: &IProfSect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProfSect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProfSect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IProfSect> for IMAPIProp {
    fn from(value: IProfSect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProfSect> for IMAPIProp {
    fn from(value: &IProfSect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IProfSect {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IProfSect {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProfSect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPropData(pub ::windows::core::IUnknown);
impl IPropData {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulproptag), ::core::mem::transmute(lpiid), ::core::mem::transmute(ulinterfaceoptions), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppunk)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, Param4: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param4, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ciidexclude),
            ::core::mem::transmute(rgiidexclude),
            ::core::mem::transmute(lpexcludeprops),
            ::core::mem::transmute(uluiparam),
            lpprogress.into_param().abi(),
            ::core::mem::transmute(lpinterface),
            ::core::mem::transmute(lpdestobj),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppproblems),
        )
        .ok()
    }
    pub unsafe fn CopyProps<'a, Param2: ::windows::core::IntoParam<'a, IMAPIProgress>>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Param2, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpincludeprops), ::core::mem::transmute(uluiparam), lpprogress.into_param().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproblems)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropnames), ::core::mem::transmute(lpppropnames), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn HrSetObjAccess(&self, ulaccess: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulaccess)).ok()
    }
    pub unsafe fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(rgulaccess)).ok()
    }
    pub unsafe fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptagarray), ::core::mem::transmute(lprgulaccess)).ok()
    }
    pub unsafe fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppproptagarray), ::core::mem::transmute(lprgulaccess)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPropData {
    type Vtable = IPropData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IPropData> for ::windows::core::IUnknown {
    fn from(value: IPropData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropData> for ::windows::core::IUnknown {
    fn from(value: &IPropData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPropData> for IMAPIProp {
    fn from(value: IPropData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropData> for IMAPIProp {
    fn from(value: &IPropData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for IPropData {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMAPIProp> for &IPropData {
    fn into_param(self) -> ::windows::core::Param<'a, IMAPIProp> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulaccess: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProviderAdmin(pub ::windows::core::IUnknown);
impl IProviderAdmin {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR> {
        let mut result__: <*mut MAPIERROR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), &mut result__).from_abi::<*mut MAPIERROR>(result__)
    }
    pub unsafe fn GetProviderTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__: <IMAPITable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IMAPITable>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateProvider(&self, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<MAPIUID> {
        let mut result__: <MAPIUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpszprovider), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpprops), ::core::mem::transmute(uluiparam), ::core::mem::transmute(ulflags), &mut result__).from_abi::<MAPIUID>(result__)
    }
    pub unsafe fn DeleteProvider(&self, lpuid: *const MAPIUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpuid)).ok()
    }
    pub unsafe fn OpenProfileSection(&self, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IProfSect> {
        let mut result__: <IProfSect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpuid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(ulflags), &mut result__).from_abi::<IProfSect>(result__)
    }
}
unsafe impl ::windows::core::Interface for IProviderAdmin {
    type Vtable = IProviderAdmin_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IProviderAdmin> for ::windows::core::IUnknown {
    fn from(value: IProviderAdmin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProviderAdmin> for ::windows::core::IUnknown {
    fn from(value: &IProviderAdmin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProviderAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProviderAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderAdmin_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpuid: *const MAPIUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppprofsect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITableData(pub ::windows::core::IUnknown);
impl ITableData {
    pub unsafe fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut ::core::option::Option<CALLERRELEASE>, ulcallerdata: u32, lppmapitable: *mut ::core::option::Option<IMAPITable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpssortorderset), ::core::mem::transmute(lpfcallerrelease), ::core::mem::transmute(ulcallerdata), ::core::mem::transmute(lppmapitable)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrModifyRow(&self, param0: *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpspropvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpspropvalue), ::core::mem::transmute(lppsrow), ::core::mem::transmute(lpulirow)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulrownumber), ::core::mem::transmute(lppsrow)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpspropvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulirow), ::core::mem::transmute(lpsrow)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpsrowset)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(lprowsettodelete), ::core::mem::transmute(crowsdeleted)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITableData {
    type Vtable = ITableData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<ITableData> for ::windows::core::IUnknown {
    fn from(value: ITableData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITableData> for ::windows::core::IUnknown {
    fn from(value: &ITableData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITableData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITableData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut ::windows::core::RawPtr, ulcallerdata: u32, lppmapitable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, param0: *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWABExtInit(pub ::windows::core::IUnknown);
impl IWABExtInit {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpwabextdisplay)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWABExtInit {
    type Vtable = IWABExtInit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea22ebf0_87a4_11d1_9acf_00a0c91f9c8b);
}
impl ::core::convert::From<IWABExtInit> for ::windows::core::IUnknown {
    fn from(value: IWABExtInit) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWABExtInit> for ::windows::core::IUnknown {
    fn from(value: &IWABExtInit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWABExtInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWABExtInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABExtInit_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpwabextdisplay: *mut ::core::mem::ManuallyDrop<WABEXTDISPLAY>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWABOBJECT_(pub ::windows::core::IUnknown);
impl IWABOBJECT_ {
    pub unsafe fn QueryInterface(&self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Release(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(lpobject), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), lpfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpwip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), lpwip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Find<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, hwnd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardDisplay<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, hwnd: Param1, lpszfilename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi(), lpszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LDAPUrl<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, hwnd: Param1, ulflags: u32, lpszurl: Param3) -> ::windows::core::Result<IMailUser> {
        let mut result__: <IMailUser as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(ulflags), lpszurl.into_param().abi(), &mut result__).from_abi::<IMailUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardCreate<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, IMailUser>>(&self, lpiab: Param0, ulflags: u32, lpszvcard: Param2, lpmailuser: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), lpszvcard.into_param().abi(), lpmailuser.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardRetrieve<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, ulflags: u32, lpszvcard: Param2) -> ::windows::core::Result<IMailUser> {
        let mut result__: <IMailUser as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), lpszvcard.into_param().abi(), &mut result__).from_abi::<IMailUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMe<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpdwaction), ::core::mem::transmute(lpsbeid), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMe<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, SBinary>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, ulflags: u32, sbeid: Param2, hwnd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), sbeid.into_param().abi(), hwnd.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWABOBJECT_ {
    type Vtable = IWABOBJECT__abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IWABOBJECT_> for ::windows::core::IUnknown {
    fn from(value: IWABOBJECT_) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWABOBJECT_> for ::windows::core::IUnknown {
    fn from(value: &IWABOBJECT_) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWABOBJECT_ {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWABOBJECT_ {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABOBJECT__abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub type IWABOBJECT_AddRef_METHOD = unsafe extern "system" fn() -> u32;
pub type IWABOBJECT_AllocateBuffer_METHOD = unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
pub type IWABOBJECT_AllocateMore_METHOD = unsafe extern "system" fn(cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Backup_METHOD = unsafe extern "system" fn(lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Find_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
pub type IWABOBJECT_FreeBuffer_METHOD = unsafe extern "system" fn(lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
pub type IWABOBJECT_GetLastError_METHOD = unsafe extern "system" fn(hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_GetMe_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Import_METHOD = unsafe extern "system" fn(lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_LDAPUrl_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
pub type IWABOBJECT_QueryInterface_METHOD = unsafe extern "system" fn(riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
pub type IWABOBJECT_Release_METHOD = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_SetMe_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardCreate_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardDisplay_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardRetrieve_METHOD = unsafe extern "system" fn(lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWABObject(pub ::windows::core::IUnknown);
impl IWABObject {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbsize), ::core::mem::transmute(lpobject), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), lpfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpwip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpwip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Find<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, hwnd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardDisplay<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, hwnd: Param1, lpszfilename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi(), lpszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LDAPUrl<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, hwnd: Param1, ulflags: u32, lpszurl: Param3) -> ::windows::core::Result<IMailUser> {
        let mut result__: <IMailUser as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(ulflags), lpszurl.into_param().abi(), &mut result__).from_abi::<IMailUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardCreate<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, IMailUser>>(&self, lpiab: Param0, ulflags: u32, lpszvcard: Param2, lpmailuser: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), lpszvcard.into_param().abi(), lpmailuser.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardRetrieve<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, lpiab: Param0, ulflags: u32, lpszvcard: Param2) -> ::windows::core::Result<IMailUser> {
        let mut result__: <IMailUser as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), lpszvcard.into_param().abi(), &mut result__).from_abi::<IMailUser>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMe<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpdwaction), ::core::mem::transmute(lpsbeid), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMe<'a, Param0: ::windows::core::IntoParam<'a, IAddrBook>, Param2: ::windows::core::IntoParam<'a, SBinary>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, lpiab: Param0, ulflags: u32, sbeid: Param2, hwnd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), lpiab.into_param().abi(), ::core::mem::transmute(ulflags), sbeid.into_param().abi(), hwnd.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWABObject {
    type Vtable = IWABObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
impl ::core::convert::From<IWABObject> for ::windows::core::IUnknown {
    fn from(value: IWABObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWABObject> for ::windows::core::IUnknown {
    fn from(value: &IWABObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWABObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWABObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub type LPALLOCATEBUFFER = unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPALLOCATEMORE = unsafe extern "system" fn(cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPCREATECONVERSATIONINDEX = unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
pub type LPDISPATCHNOTIFICATIONS = unsafe extern "system" fn(ulflags: u32) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNABSDI = unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type LPFNBUTTON = unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32;
pub type LPFNDISMISS = unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void);
pub type LPFREEBUFFER = unsafe extern "system" fn(lpbuffer: *mut ::core::ffi::c_void) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNOTIFCALLBACK = unsafe extern "system" fn(lpvcontext: *mut ::core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32;
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = unsafe extern "system" fn(lpallocatebuffer: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
pub type LPWABALLOCATEBUFFER = unsafe extern "system" fn(lpwabobject: ::windows::core::RawPtr, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPWABALLOCATEMORE = unsafe extern "system" fn(lpwabobject: ::windows::core::RawPtr, cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPWABFREEBUFFER = unsafe extern "system" fn(lpwabobject: ::windows::core::RawPtr, lpbuffer: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPEN = unsafe extern "system" fn(lppadrbook: *mut ::windows::core::RawPtr, lppwabobject: *mut ::windows::core::RawPtr, lpwp: *mut WAB_PARAM, reserved2: u32) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPENEX = unsafe extern "system" fn(lppadrbook: *mut ::windows::core::RawPtr, lppwabobject: *mut ::windows::core::RawPtr, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: ::windows::core::RawPtr, fnallocatemore: ::windows::core::RawPtr, fnfreebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32;
        }
        ::core::mem::transmute(LPropCompareProp(::core::mem::transmute(lpspropvaluea), ::core::mem::transmute(lpspropvalueb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue;
        }
        ::core::mem::transmute(LpValFindProp(::core::mem::transmute(ulproptag), ::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MAPIDeinitIdle() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MAPIDeinitIdle();
        }
        ::core::mem::transmute(MAPIDeinitIdle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
}
impl MAPIERROR {}
impl ::core::default::Default for MAPIERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MAPIERROR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MAPIERROR").field("ulVersion", &self.ulVersion).field("lpszError", &self.lpszError).field("lpszComponent", &self.lpszComponent).field("ulLowLevelError", &self.ulLowLevelError).field("ulContext", &self.ulContext).finish()
    }
}
impl ::core::cmp::PartialEq for MAPIERROR {
    fn eq(&self, other: &Self) -> bool {
        self.ulVersion == other.ulVersion && self.lpszError == other.lpszError && self.lpszComponent == other.lpszComponent && self.ulLowLevelError == other.ulLowLevelError && self.ulContext == other.ulContext
    }
}
impl ::core::cmp::Eq for MAPIERROR {}
unsafe impl ::windows::core::Abi for MAPIERROR {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc>;
        }
        ::core::mem::transmute(MAPIGetDefaultMalloc())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(MAPIInitIdle(::core::mem::transmute(lpvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MAPINAMEID {
    pub lpguid: *mut ::windows::core::GUID,
    pub ulKind: u32,
    pub Kind: MAPINAMEID_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MAPINAMEID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MAPINAMEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MAPINAMEID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MAPINAMEID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MAPINAMEID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MAPINAMEID_0 {
    pub lID: i32,
    pub lpwstrName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MAPINAMEID_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MAPINAMEID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MAPINAMEID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MAPINAMEID_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MAPINAMEID_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
impl MAPIUID {}
impl ::core::default::Default for MAPIUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MAPIUID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MAPIUID").field("ab", &self.ab).finish()
    }
}
impl ::core::cmp::PartialEq for MAPIUID {
    fn eq(&self, other: &Self) -> bool {
        self.ab == other.ab
    }
}
impl ::core::cmp::Eq for MAPIUID {}
unsafe impl ::windows::core::Abi for MAPIUID {
    type Abi = Self;
}
pub const MAPI_COMPOUND: u32 = 128u32;
pub const MAPI_DIM: u32 = 1u32;
pub const MAPI_ERROR_VERSION: i32 = 0i32;
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
pub const MAPI_NOTRECIP: u32 = 64u32;
pub const MAPI_NOTRESERVED: u32 = 8u32;
pub const MAPI_NOW: u32 = 16u32;
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
pub const MAPI_P1: u32 = 268435456u32;
pub const MAPI_SHORTTERM: u32 = 128u32;
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
pub const MAPI_THISSESSION: u32 = 32u32;
pub const MAPI_USE_DEFAULT: u32 = 64u32;
pub const MNID_ID: u32 = 0u32;
pub const MNID_STRING: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl MTSID {}
impl ::core::default::Default for MTSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MTSID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MTSID").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
impl ::core::cmp::PartialEq for MTSID {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for MTSID {}
unsafe impl ::windows::core::Abi for MTSID {
    type Abi = Self;
}
pub const MV_FLAG: u32 = 4096u32;
pub const MV_INSTANCE: u32 = 8192u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
}
impl NEWMAIL_NOTIFICATION {}
impl ::core::default::Default for NEWMAIL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NEWMAIL_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NEWMAIL_NOTIFICATION")
            .field("cbEntryID", &self.cbEntryID)
            .field("lpEntryID", &self.lpEntryID)
            .field("cbParentID", &self.cbParentID)
            .field("lpParentID", &self.lpParentID)
            .field("ulFlags", &self.ulFlags)
            .field("lpszMessageClass", &self.lpszMessageClass)
            .field("ulMessageFlags", &self.ulMessageFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NEWMAIL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.cbParentID == other.cbParentID && self.lpParentID == other.lpParentID && self.ulFlags == other.ulFlags && self.lpszMessageClass == other.lpszMessageClass && self.ulMessageFlags == other.ulMessageFlags
    }
}
impl ::core::cmp::Eq for NEWMAIL_NOTIFICATION {}
unsafe impl ::windows::core::Abi for NEWMAIL_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct NOTIFICATION {
    pub ulEventType: u32,
    pub ulAlignPad: u32,
    pub info: NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union NOTIFICATION_0 {
    pub err: ERROR_NOTIFICATION,
    pub newmail: NEWMAIL_NOTIFICATION,
    pub obj: OBJECT_NOTIFICATION,
    pub tab: TABLE_NOTIFICATION,
    pub ext: EXTENDED_NOTIFICATION,
    pub statobj: STATUS_OBJECT_NOTIFICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for NOTIFICATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for NOTIFICATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl NOTIFKEY {}
impl ::core::default::Default for NOTIFKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NOTIFKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NOTIFKEY").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
impl ::core::cmp::PartialEq for NOTIFKEY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.ab == other.ab
    }
}
impl ::core::cmp::Eq for NOTIFKEY {}
unsafe impl ::windows::core::Abi for NOTIFKEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub ulObjType: u32,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub cbOldID: u32,
    pub lpOldID: *mut ENTRYID,
    pub cbOldParentID: u32,
    pub lpOldParentID: *mut ENTRYID,
    pub lpPropTagArray: *mut SPropTagArray,
}
impl OBJECT_NOTIFICATION {}
impl ::core::default::Default for OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OBJECT_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OBJECT_NOTIFICATION")
            .field("cbEntryID", &self.cbEntryID)
            .field("lpEntryID", &self.lpEntryID)
            .field("ulObjType", &self.ulObjType)
            .field("cbParentID", &self.cbParentID)
            .field("lpParentID", &self.lpParentID)
            .field("cbOldID", &self.cbOldID)
            .field("lpOldID", &self.lpOldID)
            .field("cbOldParentID", &self.cbOldParentID)
            .field("lpOldParentID", &self.lpOldParentID)
            .field("lpPropTagArray", &self.lpPropTagArray)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.ulObjType == other.ulObjType && self.cbParentID == other.cbParentID && self.lpParentID == other.lpParentID && self.cbOldID == other.cbOldID && self.lpOldID == other.lpOldID && self.cbOldParentID == other.cbOldParentID && self.lpOldParentID == other.lpOldParentID && self.lpPropTagArray == other.lpPropTagArray
    }
}
impl ::core::cmp::Eq for OBJECT_NOTIFICATION {}
unsafe impl ::windows::core::Abi for OBJECT_NOTIFICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenStreamOnFile(lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lpfreebuffer: ::core::option::Option<LPFREEBUFFER>, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8) -> ::windows::core::Result<super::Com::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenStreamOnFile(lpallocatebuffer: ::windows::core::RawPtr, lpfreebuffer: ::windows::core::RawPtr, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        OpenStreamOnFile(::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpszfilename), ::core::mem::transmute(lpszprefix), &mut result__).from_abi::<super::Com::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PFNIDLE = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
pub const PRIHIGHEST: u32 = 32767u32;
pub const PRILOWEST: i32 = -32768i32;
pub const PRIUSER: u32 = 0u32;
pub const PROP_ID_INVALID: u32 = 65535u32;
pub const PROP_ID_NULL: u32 = 0u32;
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
        }
        ::core::mem::transmute(PpropFindProp(::core::mem::transmute(lpproparray), ::core::mem::transmute(cvalues), ::core::mem::transmute(ulproptag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: ::core::option::Option<LPALLOCATEMORE>, lpvobject: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: ::windows::core::RawPtr, lpvobject: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(PropCopyMore(::core::mem::transmute(lpspropvaluedest), ::core::mem::transmute(lpspropvaluesrc), ::core::mem::transmute(lpfallocmore), ::core::mem::transmute(lpvobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RTFSync<'a, Param0: ::windows::core::IntoParam<'a, IMessage>>(lpmessage: Param0, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RTFSync(lpmessage: ::windows::core::RawPtr, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        RTFSync(lpmessage.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(lpfmessageupdated)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAndRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SAndRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAndRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SAndRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cRes == other.cRes && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SAndRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
impl SAppTimeArray {}
impl ::core::default::Default for SAppTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SAppTimeArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAppTimeArray").field("cValues", &self.cValues).field("lpat", &self.lpat).finish()
    }
}
impl ::core::cmp::PartialEq for SAppTimeArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpat == other.lpat
    }
}
impl ::core::cmp::Eq for SAppTimeArray {}
unsafe impl ::windows::core::Abi for SAppTimeArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
impl SBinary {}
impl ::core::default::Default for SBinary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SBinary {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SBinary").field("cb", &self.cb).field("lpb", &self.lpb).finish()
    }
}
impl ::core::cmp::PartialEq for SBinary {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpb == other.lpb
    }
}
impl ::core::cmp::Eq for SBinary {}
unsafe impl ::windows::core::Abi for SBinary {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
impl SBinaryArray {}
impl ::core::default::Default for SBinaryArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SBinaryArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SBinaryArray").field("cValues", &self.cValues).field("lpbin", &self.lpbin).finish()
    }
}
impl ::core::cmp::PartialEq for SBinaryArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpbin == other.lpbin
    }
}
impl ::core::cmp::Eq for SBinaryArray {}
unsafe impl ::windows::core::Abi for SBinaryArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
impl SBitMaskRestriction {}
impl ::core::default::Default for SBitMaskRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SBitMaskRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SBitMaskRestriction").field("relBMR", &self.relBMR).field("ulPropTag", &self.ulPropTag).field("ulMask", &self.ulMask).finish()
    }
}
impl ::core::cmp::PartialEq for SBitMaskRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relBMR == other.relBMR && self.ulPropTag == other.ulPropTag && self.ulMask == other.ulMask
    }
}
impl ::core::cmp::Eq for SBitMaskRestriction {}
unsafe impl ::windows::core::Abi for SBitMaskRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SCommentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SCommentRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCommentRestriction").field("cValues", &self.cValues).field("lpRes", &self.lpRes).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SCommentRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpRes == other.lpRes && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SCommentRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
impl SComparePropsRestriction {}
impl ::core::default::Default for SComparePropsRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SComparePropsRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SComparePropsRestriction").field("relop", &self.relop).field("ulPropTag1", &self.ulPropTag1).field("ulPropTag2", &self.ulPropTag2).finish()
    }
}
impl ::core::cmp::PartialEq for SComparePropsRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag1 == other.ulPropTag1 && self.ulPropTag2 == other.ulPropTag2
    }
}
impl ::core::cmp::Eq for SComparePropsRestriction {}
unsafe impl ::windows::core::Abi for SComparePropsRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SContentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SContentRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SContentRestriction").field("ulFuzzyLevel", &self.ulFuzzyLevel).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SContentRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulFuzzyLevel == other.ulFuzzyLevel && self.ulPropTag == other.ulPropTag && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SContentRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[cfg(feature = "Win32_System_Com")]
impl SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SCurrencyArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SCurrencyArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCurrencyArray").field("cValues", &self.cValues).field("lpcur", &self.lpcur).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SCurrencyArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpcur == other.lpcur
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for SCurrencyArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SDateTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SDateTimeArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SDateTimeArray").field("cValues", &self.cValues).field("lpft", &self.lpft).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SDateTimeArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpft == other.lpft
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SDateTimeArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
impl SDoubleArray {}
impl ::core::default::Default for SDoubleArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SDoubleArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SDoubleArray").field("cValues", &self.cValues).field("lpdbl", &self.lpdbl).finish()
    }
}
impl ::core::cmp::PartialEq for SDoubleArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpdbl == other.lpdbl
    }
}
impl ::core::cmp::Eq for SDoubleArray {}
unsafe impl ::windows::core::Abi for SDoubleArray {
    type Abi = Self;
}
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
impl SExistRestriction {}
impl ::core::default::Default for SExistRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SExistRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SExistRestriction").field("ulReserved1", &self.ulReserved1).field("ulPropTag", &self.ulPropTag).field("ulReserved2", &self.ulReserved2).finish()
    }
}
impl ::core::cmp::PartialEq for SExistRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved1 == other.ulReserved1 && self.ulPropTag == other.ulPropTag && self.ulReserved2 == other.ulReserved2
    }
}
impl ::core::cmp::Eq for SExistRestriction {}
unsafe impl ::windows::core::Abi for SExistRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut ::windows::core::GUID,
}
impl SGuidArray {}
impl ::core::default::Default for SGuidArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SGuidArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SGuidArray").field("cValues", &self.cValues).field("lpguid", &self.lpguid).finish()
    }
}
impl ::core::cmp::PartialEq for SGuidArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpguid == other.lpguid
    }
}
impl ::core::cmp::Eq for SGuidArray {}
unsafe impl ::windows::core::Abi for SGuidArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SLPSTRArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SLPSTRArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SLPSTRArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SLPSTRArray").field("cValues", &self.cValues).field("lppszA", &self.lppszA).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SLPSTRArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lppszA == other.lppszA
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SLPSTRArray {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SLPSTRArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
impl SLargeIntegerArray {}
impl ::core::default::Default for SLargeIntegerArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SLargeIntegerArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SLargeIntegerArray").field("cValues", &self.cValues).field("lpli", &self.lpli).finish()
    }
}
impl ::core::cmp::PartialEq for SLargeIntegerArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpli == other.lpli
    }
}
impl ::core::cmp::Eq for SLargeIntegerArray {}
unsafe impl ::windows::core::Abi for SLargeIntegerArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
impl SLongArray {}
impl ::core::default::Default for SLongArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SLongArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SLongArray").field("cValues", &self.cValues).field("lpl", &self.lpl).finish()
    }
}
impl ::core::cmp::PartialEq for SLongArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpl == other.lpl
    }
}
impl ::core::cmp::Eq for SLongArray {}
unsafe impl ::windows::core::Abi for SLongArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SNotRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SNotRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SNotRestriction").field("ulReserved", &self.ulReserved).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SNotRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SNotRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SOrRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SOrRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOrRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SOrRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.cRes == other.cRes && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SOrRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl SPropProblem {}
impl ::core::default::Default for SPropProblem {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SPropProblem {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SPropProblem").field("ulIndex", &self.ulIndex).field("ulPropTag", &self.ulPropTag).field("scode", &self.scode).finish()
    }
}
impl ::core::cmp::PartialEq for SPropProblem {
    fn eq(&self, other: &Self) -> bool {
        self.ulIndex == other.ulIndex && self.ulPropTag == other.ulPropTag && self.scode == other.scode
    }
}
impl ::core::cmp::Eq for SPropProblem {}
unsafe impl ::windows::core::Abi for SPropProblem {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
impl SPropProblemArray {}
impl ::core::default::Default for SPropProblemArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SPropProblemArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SPropProblemArray").field("cProblem", &self.cProblem).field("aProblem", &self.aProblem).finish()
    }
}
impl ::core::cmp::PartialEq for SPropProblemArray {
    fn eq(&self, other: &Self) -> bool {
        self.cProblem == other.cProblem && self.aProblem == other.aProblem
    }
}
impl ::core::cmp::Eq for SPropProblemArray {}
unsafe impl ::windows::core::Abi for SPropProblemArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
}
impl SPropTagArray {}
impl ::core::default::Default for SPropTagArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SPropTagArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SPropTagArray").field("cValues", &self.cValues).field("aulPropTag", &self.aulPropTag).finish()
    }
}
impl ::core::cmp::PartialEq for SPropTagArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.aulPropTag == other.aulPropTag
    }
}
impl ::core::cmp::Eq for SPropTagArray {}
unsafe impl ::windows::core::Abi for SPropTagArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropValue {
    pub ulPropTag: u32,
    pub dwAlignPad: u32,
    pub Value: _PV,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SPropValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SPropValue {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SPropValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SPropValue {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropertyRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SPropertyRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SPropertyRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SPropertyRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag == other.ulPropTag && self.lpProp == other.lpProp
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SPropertyRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
}
impl SRealArray {}
impl ::core::default::Default for SRealArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SRealArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SRealArray").field("cValues", &self.cValues).field("lpflt", &self.lpflt).finish()
    }
}
impl ::core::cmp::PartialEq for SRealArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpflt == other.lpflt
    }
}
impl ::core::cmp::Eq for SRealArray {}
unsafe impl ::windows::core::Abi for SRealArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRestriction {
    pub rt: u32,
    pub res: SRestriction_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRestriction {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union SRestriction_0 {
    pub resCompareProps: SComparePropsRestriction,
    pub resAnd: SAndRestriction,
    pub resOr: SOrRestriction,
    pub resNot: SNotRestriction,
    pub resContent: SContentRestriction,
    pub resProperty: SPropertyRestriction,
    pub resBitMask: SBitMaskRestriction,
    pub resSize: SSizeRestriction,
    pub resExist: SExistRestriction,
    pub resSub: SSubRestriction,
    pub resComment: SCommentRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SRestriction_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRestriction_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRestriction_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRestriction_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRestriction_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRow {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRow {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SRow").field("ulAdrEntryPad", &self.ulAdrEntryPad).field("cValues", &self.cValues).field("lpProps", &self.lpProps).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRow {
    fn eq(&self, other: &Self) -> bool {
        self.ulAdrEntryPad == other.ulAdrEntryPad && self.cValues == other.cValues && self.lpProps == other.lpProps
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRow {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRowSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRowSet {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SRowSet").field("cRows", &self.cRows).field("aRow", &self.aRow).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRowSet {
    fn eq(&self, other: &Self) -> bool {
        self.cRows == other.cRows && self.aRow == other.aRow
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRowSet {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
impl SShortArray {}
impl ::core::default::Default for SShortArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SShortArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SShortArray").field("cValues", &self.cValues).field("lpi", &self.lpi).finish()
    }
}
impl ::core::cmp::PartialEq for SShortArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lpi == other.lpi
    }
}
impl ::core::cmp::Eq for SShortArray {}
unsafe impl ::windows::core::Abi for SShortArray {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
impl SSizeRestriction {}
impl ::core::default::Default for SSizeRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SSizeRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SSizeRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("cb", &self.cb).finish()
    }
}
impl ::core::cmp::PartialEq for SSizeRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.relop == other.relop && self.ulPropTag == other.ulPropTag && self.cb == other.cb
    }
}
impl ::core::cmp::Eq for SSizeRestriction {}
unsafe impl ::windows::core::Abi for SSizeRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
impl SSortOrder {}
impl ::core::default::Default for SSortOrder {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SSortOrder {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SSortOrder").field("ulPropTag", &self.ulPropTag).field("ulOrder", &self.ulOrder).finish()
    }
}
impl ::core::cmp::PartialEq for SSortOrder {
    fn eq(&self, other: &Self) -> bool {
        self.ulPropTag == other.ulPropTag && self.ulOrder == other.ulOrder
    }
}
impl ::core::cmp::Eq for SSortOrder {}
unsafe impl ::windows::core::Abi for SSortOrder {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
impl SSortOrderSet {}
impl ::core::default::Default for SSortOrderSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SSortOrderSet {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SSortOrderSet").field("cSorts", &self.cSorts).field("cCategories", &self.cCategories).field("cExpanded", &self.cExpanded).field("aSort", &self.aSort).finish()
    }
}
impl ::core::cmp::PartialEq for SSortOrderSet {
    fn eq(&self, other: &Self) -> bool {
        self.cSorts == other.cSorts && self.cCategories == other.cCategories && self.cExpanded == other.cExpanded && self.aSort == other.aSort
    }
}
impl ::core::cmp::Eq for SSortOrderSet {}
unsafe impl ::windows::core::Abi for SSortOrderSet {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SSubRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SSubRestriction {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SSubRestriction").field("ulSubObject", &self.ulSubObject).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SSubRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.ulSubObject == other.ulSubObject && self.lpRes == other.lpRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SSubRestriction {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for STATUS_OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for STATUS_OBJECT_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("STATUS_OBJECT_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("cValues", &self.cValues).field("lpPropVals", &self.lpPropVals).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for STATUS_OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbEntryID == other.cbEntryID && self.lpEntryID == other.lpEntryID && self.cValues == other.cValues && self.lpPropVals == other.lpPropVals
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for STATUS_OBJECT_NOTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SWStringArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWStringArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SWStringArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SWStringArray").field("cValues", &self.cValues).field("lppszW", &self.lppszW).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWStringArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.lppszW == other.lppszW
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWStringArray {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWStringArray {
    type Abi = Self;
}
pub const S_IMAPI_BOTHADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141126i32 as _);
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(11141632i32 as _);
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(11143688i32 as _);
pub const S_IMAPI_ROTATIONADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141125i32 as _);
pub const S_IMAPI_SPEEDADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141124i32 as _);
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(11141890i32 as _);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScCopyNotifications(::core::mem::transmute(cnotification), ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpvdst), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScCopyProps(::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpvdst), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScCountNotifications(::core::mem::transmute(cnotifications), ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScCountProps(::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
        }
        ::core::mem::transmute(ScCreateConversationIndex(::core::mem::transmute(cbparent), ::core::mem::transmute(lpbparent), ::core::mem::transmute(lpcbconvindex), ::core::mem::transmute(lppbconvindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: ::core::option::Option<LPALLOCATEBUFFER>, lppproparray: *mut *mut SPropValue) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: ::windows::core::RawPtr, lppproparray: *mut *mut SPropValue) -> i32;
        }
        ::core::mem::transmute(ScDupPropset(::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lppproparray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ScInitMapiUtil(ulflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScInitMapiUtil(ulflags: u32) -> i32;
        }
        ::core::mem::transmute(ScInitMapiUtil(::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScLocalPathFromUNC<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszunc: Param0, lpszlocal: Param1, cchlocal: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScLocalPathFromUNC(lpszunc: super::super::Foundation::PSTR, lpszlocal: super::super::Foundation::PSTR, cchlocal: u32) -> i32;
        }
        ::core::mem::transmute(ScLocalPathFromUNC(lpszunc.into_param().abi(), lpszlocal.into_param().abi(), ::core::mem::transmute(cchlocal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScRelocNotifications(::core::mem::transmute(cnotification), ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpvbaseold), ::core::mem::transmute(lpvbasenew), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
        }
        ::core::mem::transmute(ScRelocProps(::core::mem::transmute(cvalues), ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpvbaseold), ::core::mem::transmute(lpvbasenew), ::core::mem::transmute(lpcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScUNCFromLocalPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszlocal: Param0, lpszunc: Param1, cchunc: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScUNCFromLocalPath(lpszlocal: super::super::Foundation::PSTR, lpszunc: super::super::Foundation::PSTR, cchunc: u32) -> i32;
        }
        ::core::mem::transmute(ScUNCFromLocalPath(lpszlocal.into_param().abi(), lpszunc.into_param().abi(), ::core::mem::transmute(cchunc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8;
        }
        ::core::mem::transmute(SzFindCh(::core::mem::transmute(lpsz), ::core::mem::transmute(ch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8;
        }
        ::core::mem::transmute(SzFindLastCh(::core::mem::transmute(lpsz), ::core::mem::transmute(ch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8;
        }
        ::core::mem::transmute(SzFindSz(::core::mem::transmute(lpsz), ::core::mem::transmute(lpszkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TABLE_CHANGED: u32 = 1u32;
pub const TABLE_ERROR: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct TABLE_NOTIFICATION {
    pub ulTableEvent: u32,
    pub hResult: ::windows::core::HRESULT,
    pub propIndex: SPropValue,
    pub propPrior: SPropValue,
    pub row: SRow,
    pub ulPad: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl TABLE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for TABLE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for TABLE_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for TABLE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for TABLE_NOTIFICATION {
    type Abi = Self;
}
pub const TABLE_RELOAD: u32 = 9u32;
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
pub const TABLE_ROW_ADDED: u32 = 3u32;
pub const TABLE_ROW_DELETED: u32 = 4u32;
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
pub const TABLE_SETCOL_DONE: u32 = 8u32;
pub const TABLE_SORT_DONE: u32 = 6u32;
pub const TAD_ALL_ROWS: u32 = 1u32;
#[inline]
pub unsafe fn UFromSz(lpsz: *mut i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UFromSz(lpsz: *mut i8) -> u32;
        }
        ::core::mem::transmute(UFromSz(::core::mem::transmute(lpsz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
pub const UI_SERVICE: u32 = 2u32;
#[inline]
pub unsafe fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(UlAddRef(::core::mem::transmute(lpunk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32;
        }
        ::core::mem::transmute(UlPropSize(::core::mem::transmute(lpspropvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(UlRelease(::core::mem::transmute(lpunk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WABEXTDISPLAY {
    pub cbSize: u32,
    pub lpWABObject: ::core::option::Option<IWABObject>,
    pub lpAdrBook: ::core::option::Option<IAddrBook>,
    pub lpPropObj: ::core::option::Option<IMAPIProp>,
    pub fReadOnly: super::super::Foundation::BOOL,
    pub fDataChanged: super::super::Foundation::BOOL,
    pub ulFlags: u32,
    pub lpv: *mut ::core::ffi::c_void,
    pub lpsz: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl WABEXTDISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WABEXTDISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABEXTDISPLAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WABEXTDISPLAY")
            .field("cbSize", &self.cbSize)
            .field("lpWABObject", &self.lpWABObject)
            .field("lpAdrBook", &self.lpAdrBook)
            .field("lpPropObj", &self.lpPropObj)
            .field("fReadOnly", &self.fReadOnly)
            .field("fDataChanged", &self.fDataChanged)
            .field("ulFlags", &self.ulFlags)
            .field("lpv", &self.lpv)
            .field("lpsz", &self.lpsz)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WABEXTDISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpWABObject == other.lpWABObject && self.lpAdrBook == other.lpAdrBook && self.lpPropObj == other.lpPropObj && self.fReadOnly == other.fReadOnly && self.fDataChanged == other.fDataChanged && self.ulFlags == other.ulFlags && self.lpv == other.lpv && self.lpsz == other.lpsz
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WABEXTDISPLAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WABEXTDISPLAY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: ::core::option::Option<IAddrBook>,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WABIMPORTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WABIMPORTPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABIMPORTPARAM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WABIMPORTPARAM").field("cbSize", &self.cbSize).field("lpAdrBook", &self.lpAdrBook).field("hWnd", &self.hWnd).field("ulFlags", &self.ulFlags).field("lpszFileName", &self.lpszFileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WABIMPORTPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpAdrBook == other.lpAdrBook && self.hWnd == other.hWnd && self.ulFlags == other.ulFlags && self.lpszFileName == other.lpszFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WABIMPORTPARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WABIMPORTPARAM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
pub const WABOBJECT_ME_NEW: u32 = 1u32;
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: super::super::Foundation::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAB_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WAB_PARAM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WAB_PARAM").field("cbSize", &self.cbSize).field("hwnd", &self.hwnd).field("szFileName", &self.szFileName).field("ulFlags", &self.ulFlags).field("guidPSExt", &self.guidPSExt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAB_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwnd == other.hwnd && self.szFileName == other.szFileName && self.ulFlags == other.ulFlags && self.guidPSExt == other.guidPSExt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAB_PARAM {
    type Abi = Self;
}
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
pub const WAB_VCARD_FILE: u32 = 0u32;
pub const WAB_VCARD_STREAM: u32 = 1u32;
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WrapCompressedRTFStream<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IStream>>(lpcompressedrtfstream: Param0, ulflags: u32) -> ::windows::core::Result<super::Com::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WrapCompressedRTFStream(lpcompressedrtfstream: ::windows::core::RawPtr, ulflags: u32, lpuncompressedrtfstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WrapCompressedRTFStream(lpcompressedrtfstream.into_param().abi(), ::core::mem::transmute(ulflags), &mut result__).from_abi::<super::Com::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT;
        }
        WrapStoreEntryID(::core::mem::transmute(ulflags), ::core::mem::transmute(lpszdllname), ::core::mem::transmute(cborigentry), ::core::mem::transmute(lporigentry), ::core::mem::transmute(lpcbwrappedentry), ::core::mem::transmute(lppwrappedentry)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union _PV {
    pub i: i16,
    pub l: i32,
    pub ul: u32,
    pub flt: f32,
    pub dbl: f64,
    pub b: u16,
    pub cur: super::Com::CY,
    pub at: f64,
    pub ft: super::super::Foundation::FILETIME,
    pub lpszA: super::super::Foundation::PSTR,
    pub bin: SBinary,
    pub lpszW: super::super::Foundation::PWSTR,
    pub lpguid: *mut ::windows::core::GUID,
    pub li: i64,
    pub MVi: SShortArray,
    pub MVl: SLongArray,
    pub MVflt: SRealArray,
    pub MVdbl: SDoubleArray,
    pub MVcur: SCurrencyArray,
    pub MVat: SAppTimeArray,
    pub MVft: SDateTimeArray,
    pub MVbin: SBinaryArray,
    pub MVszA: SLPSTRArray,
    pub MVszW: SWStringArray,
    pub MVguid: SGuidArray,
    pub MVli: SLargeIntegerArray,
    pub err: i32,
    pub x: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl _PV {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _PV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for _PV {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for _PV {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for _PV {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct _WABACTIONITEM(pub u8);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct _flaglist {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
impl _flaglist {}
impl ::core::default::Default for _flaglist {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _flaglist {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_flaglist").field("cFlags", &self.cFlags).field("ulFlag", &self.ulFlag).finish()
    }
}
impl ::core::cmp::PartialEq for _flaglist {
    fn eq(&self, other: &Self) -> bool {
        self.cFlags == other.cFlags && self.ulFlag == other.ulFlag
    }
}
impl ::core::cmp::Eq for _flaglist {}
unsafe impl ::windows::core::Abi for _flaglist {
    type Abi = Self;
}
