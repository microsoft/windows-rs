#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADRENTRY").field("ulReserved1", &self.ulReserved1).field("cValues", &self.cValues).field("rgPropVals", &self.rgPropVals).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for ADRENTRY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADRENTRY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADRLIST").field("cEntries", &self.cEntries).field("aEntries", &self.aEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for ADRLIST {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADRLIST>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct ADRPARM {
    pub cbABContEntryID: u32,
    pub lpABContEntryID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
    pub ulHelpContext: u32,
    pub lpszHelpFileName: *mut i8,
    pub lpfnABSDI: LPFNABSDI,
    pub lpfnDismiss: LPFNDISMISS,
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
impl ::core::marker::Copy for ADRPARM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for ADRPARM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for ADRPARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADRPARM")
            .field("cbABContEntryID", &self.cbABContEntryID)
            .field("lpABContEntryID", &self.lpABContEntryID)
            .field("ulFlags", &self.ulFlags)
            .field("lpReserved", &self.lpReserved)
            .field("ulHelpContext", &self.ulHelpContext)
            .field("lpszHelpFileName", &self.lpszHelpFileName)
            .field("lpfnABSDI", &self.lpfnABSDI.map(|f| f as usize))
            .field("lpfnDismiss", &self.lpfnDismiss.map(|f| f as usize))
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
unsafe impl ::windows::core::Abi for ADRPARM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for ADRPARM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADRPARM>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for ADRPARM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for ADRPARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn BuildDisplayTable<'a, P0, P1>(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: P0, hinstance: P1, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut ::core::option::Option<IMAPITable>, lpptbldata: *mut ::core::option::Option<ITableData>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IMalloc>>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BuildDisplayTable(lpallocatebuffer: *mut ::core::ffi::c_void, lpallocatemore: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, lpmalloc: *mut ::core::ffi::c_void, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void, lpptbldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    BuildDisplayTable(::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpallocatemore), ::core::mem::transmute(lpfreebuffer), lpmalloc.into().abi(), hinstance.into(), cpages, ::core::mem::transmute(lppage), ulflags, ::core::mem::transmute(lpptable), ::core::mem::transmute(lpptbldata)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type CALLERRELEASE = ::core::option::Option<unsafe extern "system" fn(ulcallerdata: u32, lptbldata: ::core::option::Option<ITableData>, lpvue: ::core::option::Option<IMAPITable>)>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: *mut ::core::ffi::c_void, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
    }
    ChangeIdleRoutine(::core::mem::transmute(ftg), ::core::mem::transmute(lpfnidle), ::core::mem::transmute(lpvidleparam), priidle, csecidle, iroidle, ircidle)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn CreateIProp(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut ::core::option::Option<IPropData>) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIProp(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: *mut ::core::ffi::c_void, lpallocatemore: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut *mut ::core::ffi::c_void) -> i32;
    }
    CreateIProp(::core::mem::transmute(lpinterface), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpallocatemore), ::core::mem::transmute(lpfreebuffer), ::core::mem::transmute(lpvreserved), ::core::mem::transmute(lpppropdata))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn CreateTable(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ::core::option::Option<ITableData>) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateTable(lpinterface: *mut ::windows::core::GUID, lpallocatebuffer: *mut ::core::ffi::c_void, lpallocatemore: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut *mut ::core::ffi::c_void) -> i32;
    }
    CreateTable(::core::mem::transmute(lpinterface), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpallocatemore), ::core::mem::transmute(lpfreebuffer), ::core::mem::transmute(lpvreserved), ultabletype, ulproptagindexcolumn, ::core::mem::transmute(lpsproptagarraycolumns), ::core::mem::transmute(lpptabledata))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
impl ::core::marker::Copy for DTBLBUTTON {}
impl ::core::clone::Clone for DTBLBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRControl", &self.ulPRControl).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLBUTTON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLBUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLBUTTON>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLBUTTON {}
impl ::core::default::Default for DTBLBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
impl ::core::marker::Copy for DTBLCHECKBOX {}
impl ::core::clone::Clone for DTBLCHECKBOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLCHECKBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLCHECKBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulPRPropertyName", &self.ulPRPropertyName).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLCHECKBOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLCHECKBOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLCHECKBOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLCHECKBOX {}
impl ::core::default::Default for DTBLCHECKBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLCOMBOBOX {}
impl ::core::clone::Clone for DTBLCOMBOBOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLCOMBOBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLCOMBOBOX").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPRPropertyName", &self.ulPRPropertyName).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLCOMBOBOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLCOMBOBOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLCOMBOBOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLCOMBOBOX {}
impl ::core::default::Default for DTBLCOMBOBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLDDLBX {}
impl ::core::clone::Clone for DTBLDDLBX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLDDLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLDDLBX").field("ulFlags", &self.ulFlags).field("ulPRDisplayProperty", &self.ulPRDisplayProperty).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLDDLBX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLDDLBX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLDDLBX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLDDLBX {}
impl ::core::default::Default for DTBLDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
impl ::core::marker::Copy for DTBLEDIT {}
impl ::core::clone::Clone for DTBLEDIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLEDIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLEDIT").field("ulbLpszCharsAllowed", &self.ulbLpszCharsAllowed).field("ulFlags", &self.ulFlags).field("ulNumCharsAllowed", &self.ulNumCharsAllowed).field("ulPropTag", &self.ulPropTag).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLEDIT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLEDIT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLEDIT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLEDIT {}
impl ::core::default::Default for DTBLEDIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for DTBLGROUPBOX {}
impl ::core::clone::Clone for DTBLGROUPBOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLGROUPBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLGROUPBOX").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLGROUPBOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLGROUPBOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLGROUPBOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLGROUPBOX {}
impl ::core::default::Default for DTBLGROUPBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for DTBLLABEL {}
impl ::core::clone::Clone for DTBLLABEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLLABEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLLABEL").field("ulbLpszLabelName", &self.ulbLpszLabelName).field("ulFlags", &self.ulFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLLABEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLLABEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLLABEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLLABEL {}
impl ::core::default::Default for DTBLLABEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
impl ::core::marker::Copy for DTBLLBX {}
impl ::core::clone::Clone for DTBLLBX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLLBX").field("ulFlags", &self.ulFlags).field("ulPRSetProperty", &self.ulPRSetProperty).field("ulPRTableName", &self.ulPRTableName).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLLBX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLLBX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLLBX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLLBX {}
impl ::core::default::Default for DTBLLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl ::core::marker::Copy for DTBLMVDDLBX {}
impl ::core::clone::Clone for DTBLMVDDLBX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLMVDDLBX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLMVDDLBX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLMVDDLBX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLMVDDLBX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLMVDDLBX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLMVDDLBX {}
impl ::core::default::Default for DTBLMVDDLBX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
impl ::core::marker::Copy for DTBLMVLISTBOX {}
impl ::core::clone::Clone for DTBLMVLISTBOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLMVLISTBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLMVLISTBOX").field("ulFlags", &self.ulFlags).field("ulMVPropTag", &self.ulMVPropTag).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLMVLISTBOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLMVLISTBOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLMVLISTBOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLMVLISTBOX {}
impl ::core::default::Default for DTBLMVLISTBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
impl ::core::marker::Copy for DTBLPAGE {}
impl ::core::clone::Clone for DTBLPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLPAGE").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulbLpszComponent", &self.ulbLpszComponent).field("ulContext", &self.ulContext).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLPAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLPAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLPAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLPAGE {}
impl ::core::default::Default for DTBLPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
}
impl ::core::marker::Copy for DTBLRADIOBUTTON {}
impl ::core::clone::Clone for DTBLRADIOBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBLRADIOBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBLRADIOBUTTON").field("ulbLpszLabel", &self.ulbLpszLabel).field("ulFlags", &self.ulFlags).field("ulcButtons", &self.ulcButtons).field("ulPropTag", &self.ulPropTag).field("lReturnValue", &self.lReturnValue).finish()
    }
}
unsafe impl ::windows::core::Abi for DTBLRADIOBUTTON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBLRADIOBUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBLRADIOBUTTON>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBLRADIOBUTTON {}
impl ::core::default::Default for DTBLRADIOBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTCTL {
    pub ulCtlType: u32,
    pub ulCtlFlags: u32,
    pub lpbNotif: *mut u8,
    pub cbNotif: u32,
    pub lpszFilter: *mut i8,
    pub ulItemID: u32,
    pub ctl: DTCTL_0,
}
impl ::core::marker::Copy for DTCTL {}
impl ::core::clone::Clone for DTCTL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTCTL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTCTL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTCTL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTCTL {}
impl ::core::default::Default for DTCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
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
impl ::core::marker::Copy for DTCTL_0 {}
impl ::core::clone::Clone for DTCTL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTCTL_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTCTL_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTCTL_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTCTL_0 {}
impl ::core::default::Default for DTCTL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct DTPAGE {
    pub cctl: u32,
    pub lpszResourceName: *mut i8,
    pub Anonymous: DTPAGE_0,
    pub lpctl: *mut DTCTL,
}
impl ::core::marker::Copy for DTPAGE {}
impl ::core::clone::Clone for DTPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTPAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTPAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTPAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTPAGE {}
impl ::core::default::Default for DTPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub union DTPAGE_0 {
    pub lpszComponent: *mut i8,
    pub ulItemID: u32,
}
impl ::core::marker::Copy for DTPAGE_0 {}
impl ::core::clone::Clone for DTPAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTPAGE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTPAGE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTPAGE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTPAGE_0 {}
impl ::core::default::Default for DTPAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn DeinitMapiUtil() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeinitMapiUtil();
    }
    DeinitMapiUtil()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void);
    }
    DeregisterIdleRoutine(::core::mem::transmute(ftg))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for ENTRYID {}
impl ::core::clone::Clone for ENTRYID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENTRYID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENTRYID").field("abFlags", &self.abFlags).field("ab", &self.ab).finish()
    }
}
unsafe impl ::windows::core::Abi for ENTRYID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENTRYID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENTRYID>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENTRYID {}
impl ::core::default::Default for ENTRYID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
impl ::core::marker::Copy for ERROR_NOTIFICATION {}
impl ::core::clone::Clone for ERROR_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ERROR_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERROR_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("scode", &self.scode).field("ulFlags", &self.ulFlags).field("lpMAPIError", &self.lpMAPIError).finish()
    }
}
unsafe impl ::windows::core::Abi for ERROR_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ERROR_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERROR_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ERROR_NOTIFICATION {}
impl ::core::default::Default for ERROR_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
impl ::core::marker::Copy for EXTENDED_NOTIFICATION {}
impl ::core::clone::Clone for EXTENDED_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXTENDED_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENDED_NOTIFICATION").field("ulEvent", &self.ulEvent).field("cb", &self.cb).field("pbEventParameters", &self.pbEventParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for EXTENDED_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXTENDED_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTENDED_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXTENDED_NOTIFICATION {}
impl ::core::default::Default for EXTENDED_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_BURN_VERIFICATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600697i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599672i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599678i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599674i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599675i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599673i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599677i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599676i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599680i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599679i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599164i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599154i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599162i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599166i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599161i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599165i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599159i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599158i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599152i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599153i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599155i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599168i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599167i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599409i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_INVALID_ISRC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599413i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_INVALID_MCN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599412i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599418i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599422i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599417i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599421i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599415i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599414i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599420i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599410i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599411i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599419i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599416i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599424i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599423i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598389i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340222i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340219i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340216i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340220i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598391i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340221i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340223i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_RECORDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340224i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062598390i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340218i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136340217i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_LOSS_OF_STREAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599936i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339963i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339968i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339965i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339966i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339967i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339962i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339964i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339961i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339959i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2136339958i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600175i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600179i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600178i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600181i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600180i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600184i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599937i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600176i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600187i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600185i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600186i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600189i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600174i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600190i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600177i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600188i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600183i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600182i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600191i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_RECORDER_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600701i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_REQUEST_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062600702i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062599935i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableIdleRoutine<'a, P0>(ftg: *mut ::core::ffi::c_void, fenable: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableIdleRoutine(ftg: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL);
    }
    EnableIdleRoutine(::core::mem::transmute(ftg), fenable.into())
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const FACILITY_IMAPI2: u32 = 170u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL;
    }
    FEqualNames(::core::mem::transmute(lpname1), ::core::mem::transmute(lpname2))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
impl ::core::marker::Copy for FLATENTRY {}
impl ::core::clone::Clone for FLATENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLATENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATENTRY").field("cb", &self.cb).field("abEntry", &self.abEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for FLATENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLATENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLATENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLATENTRY {}
impl ::core::default::Default for FLATENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
impl ::core::marker::Copy for FLATENTRYLIST {}
impl ::core::clone::Clone for FLATENTRYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLATENTRYLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATENTRYLIST").field("cEntries", &self.cEntries).field("cbEntries", &self.cbEntries).field("abEntries", &self.abEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for FLATENTRYLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLATENTRYLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLATENTRYLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLATENTRYLIST {}
impl ::core::default::Default for FLATENTRYLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
impl ::core::marker::Copy for FLATMTSIDLIST {}
impl ::core::clone::Clone for FLATMTSIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLATMTSIDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLATMTSIDLIST").field("cMTSIDs", &self.cMTSIDs).field("cbMTSIDs", &self.cbMTSIDs).field("abMTSIDs", &self.abMTSIDs).finish()
    }
}
unsafe impl ::windows::core::Abi for FLATMTSIDLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLATMTSIDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLATMTSIDLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLATMTSIDLIST {}
impl ::core::default::Default for FLATMTSIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL;
    }
    FPropCompareProp(::core::mem::transmute(lpspropvalue1), ulrelop, ::core::mem::transmute(lpspropvalue2))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL;
    }
    FPropContainsProp(::core::mem::transmute(lpspropvaluedst), ::core::mem::transmute(lpspropvaluesrc), ulfuzzylevel)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FPropExists<'a, P0>(lpmapiprop: P0, ulproptag: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProp>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FPropExists(lpmapiprop: *mut ::core::ffi::c_void, ulproptag: u32) -> super::super::Foundation::BOOL;
    }
    FPropExists(lpmapiprop.into().abi(), ulproptag)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FreePadrlist(lpadrlist: *mut ADRLIST) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreePadrlist(lpadrlist: *mut ADRLIST);
    }
    FreePadrlist(::core::mem::transmute(lpadrlist))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn FreeProws(lprows: *mut SRowSet) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeProws(lprows: *mut SRowSet);
    }
    FreeProws(::core::mem::transmute(lprows))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    }
    FtAddFt(::core::mem::transmute(ftaddend1), ::core::mem::transmute(ftaddend2))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    }
    FtMulDw(ftmultiplier, ::core::mem::transmute(ftmultiplicand))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME;
    }
    FtMulDwDw(ftmultiplicand, ftmultiplier)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    }
    FtNegFt(::core::mem::transmute(ft))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    }
    FtSubFt(::core::mem::transmute(ftminuend), ::core::mem::transmute(ftsubtrahend))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FtgRegisterIdleRoutine(lpfnidle: *mut ::core::ffi::c_void, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
    }
    FtgRegisterIdleRoutine(::core::mem::transmute(lpfnidle), ::core::mem::transmute(lpvidleparam), priidle, csecidle, iroidle)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Gender(pub i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderUnspecified: Gender = Gender(0i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderFemale: Gender = Gender(1i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const genderMale: Gender = Gender(2i32);
impl ::core::marker::Copy for Gender {}
impl ::core::clone::Clone for Gender {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Gender {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Gender {
    type Abi = Self;
}
impl ::core::fmt::Debug for Gender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gender").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn HrAddColumns<'a, P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPITable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrAddColumns(lptbl: *mut ::core::ffi::c_void, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HrAddColumns(lptbl.into().abi(), ::core::mem::transmute(lpproptagcolumnsnew), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn HrAddColumnsEx<'a, P0>(lptbl: P0, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPITable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrAddColumnsEx(lptbl: *mut ::core::ffi::c_void, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, lpfnfiltercolumns: isize) -> ::windows::core::HRESULT;
    }
    HrAddColumnsEx(lptbl.into().abi(), ::core::mem::transmute(lpproptagcolumnsnew), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer), lpfnfiltercolumns).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut ::core::option::Option<IMAPIAdviseSink>) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrAllocAdviseSink(lpfncallback: *mut ::core::ffi::c_void, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HrAllocAdviseSink(::core::mem::transmute(lpfncallback), ::core::mem::transmute(lpvcontext), ::core::mem::transmute(lppadvisesink)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn HrDispatchNotifications(ulflags: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrDispatchNotifications(ulflags: u32) -> ::windows::core::HRESULT;
    }
    HrDispatchNotifications(ulflags).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrGetOneProp<'a, P0>(lpmapiprop: P0, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProp>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrGetOneProp(lpmapiprop: *mut ::core::ffi::c_void, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows::core::HRESULT;
    }
    HrGetOneProp(lpmapiprop.into().abi(), ulproptag, ::core::mem::transmute(lppprop)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn HrIStorageFromStream<'a, P0>(lpunkin: P0, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppstorageout: *mut ::core::option::Option<super::Com::StructuredStorage::IStorage>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrIStorageFromStream(lpunkin: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppstorageout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HrIStorageFromStream(lpunkin.into().abi(), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lppstorageout)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrQueryAllRows<'a, P0>(lptable: P0, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPITable>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrQueryAllRows(lptable: *mut ::core::ffi::c_void, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT;
    }
    HrQueryAllRows(lptable.into().abi(), ::core::mem::transmute(lpproptags), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpsortorderset), crowsmax, ::core::mem::transmute(lpprows)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn HrSetOneProp<'a, P0>(lpmapiprop: P0, lpprop: *mut SPropValue) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProp>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrSetOneProp(lpmapiprop: *mut ::core::ffi::c_void, lpprop: *mut SPropValue) -> ::windows::core::HRESULT;
    }
    HrSetOneProp(lpmapiprop.into().abi(), ::core::mem::transmute(lpprop)).ok()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn HrThisThreadAdviseSink<'a, P0>(lpadvisesink: P0) -> ::windows::core::Result<IMAPIAdviseSink>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIAdviseSink>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HrThisThreadAdviseSink(lpadvisesink: *mut ::core::ffi::c_void, lppadvisesink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HrThisThreadAdviseSink(lpadvisesink.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPIAdviseSink>(result__)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IABContainer(::windows::core::IUnknown);
impl IABContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetContentsTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetHierarchyTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSearchCriteria)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSearchCriteria)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ulcreateflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPIProp>(result__)
    }
    pub unsafe fn CopyEntries<'a, P0>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpentries), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpentries), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResolveNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpadrlist), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<_flaglist>(result__)
    }
}
impl ::core::convert::From<IABContainer> for ::windows::core::IUnknown {
    fn from(value: IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IABContainer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IABContainer> for ::windows::core::IUnknown {
    fn from(value: &IABContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IABContainer> for IMAPIProp {
    fn from(value: IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IABContainer> for &'a IMAPIProp {
    fn from(value: &'a IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IABContainer> for IMAPIProp {
    fn from(value: &IABContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IABContainer> for IMAPIContainer {
    fn from(value: IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IABContainer> for &'a IMAPIContainer {
    fn from(value: &'a IABContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IABContainer> for IMAPIContainer {
    fn from(value: &IABContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IABContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IABContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IABContainer {}
impl ::core::fmt::Debug for IABContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IABContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IABContainer {
    type Vtable = IABContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IABContainer_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveNames: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IAddrBook(::windows::core::IUnknown);
impl IAddrBook {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompareEntryIDs)(::windows::core::Interface::as_raw(self), cbentryid1, ::core::mem::transmute(lpentryid1), cbentryid2, ::core::mem::transmute(lpentryid2), ulflags, ::core::mem::transmute(lpulresult)).ok()
    }
    pub unsafe fn Advise<'a, P0>(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: P0, lpulconnection: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIAdviseSink>>,
    {
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), uleventmask, lpadvisesink.into().abi(), ::core::mem::transmute(lpulconnection)).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateOneOff)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpszname), ::core::mem::transmute(lpszadrtype), ::core::mem::transmute(lpszaddress), ulflags, ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NewEntry)(::windows::core::Interface::as_raw(self), uluiparam, ulflags, cbeidcontainer, ::core::mem::transmute(lpeidcontainer), cbeidnewentrytpl, ::core::mem::transmute(lpeidnewentrytpl), ::core::mem::transmute(lpcbeidnewentry), ::core::mem::transmute(lppeidnewentry)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResolveName)(::windows::core::Interface::as_raw(self), uluiparam, ulflags, ::core::mem::transmute(lpsznewentrytitle), ::core::mem::transmute(lpadrlist)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Address)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpuluiparam), ::core::mem::transmute(lpadrparms), ::core::mem::transmute(lppadrlist)).ok()
    }
    pub unsafe fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Details)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpuluiparam), ::core::mem::transmute(lpfndismiss), ::core::mem::transmute(lpvdismisscontext), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpfbuttoncallback), ::core::mem::transmute(lpvbuttoncontext), ::core::mem::transmute(lpszbuttontext), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RecipOptions)(::windows::core::Interface::as_raw(self), uluiparam, ulflags, ::core::mem::transmute(lprecip)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryDefaultRecipOpt)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpszadrtype), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppoptions)).ok()
    }
    pub unsafe fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPAB)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPAB)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid)).ok()
    }
    pub unsafe fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultDir)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid)).ok()
    }
    pub unsafe fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultDir)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSearchPath)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppsearchpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSearchPath)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpsearchpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareRecips)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lpreciplist)).ok()
    }
}
impl ::core::convert::From<IAddrBook> for ::windows::core::IUnknown {
    fn from(value: IAddrBook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAddrBook> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAddrBook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrBook> for ::windows::core::IUnknown {
    fn from(value: &IAddrBook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAddrBook> for IMAPIProp {
    fn from(value: IAddrBook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAddrBook> for &'a IMAPIProp {
    fn from(value: &'a IAddrBook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrBook> for IMAPIProp {
    fn from(value: &IAddrBook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAddrBook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrBook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrBook {}
impl ::core::fmt::Debug for IAddrBook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrBook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAddrBook {
    type Vtable = IAddrBook_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrBook_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub OpenEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT,
    pub CreateOneOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub NewEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Address: usize,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: *mut ::core::ffi::c_void, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: *mut ::core::ffi::c_void, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RecipOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RecipOptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryDefaultRecipOpt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryDefaultRecipOpt: usize,
    pub GetPAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub SetPAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT,
    pub GetDefaultDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT,
    pub SetDefaultDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSearchPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSearchPath: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetSearchPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetSearchPath: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PrepareRecips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PrepareRecips: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IAttach(::windows::core::IUnknown);
impl IAttach {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
}
impl ::core::convert::From<IAttach> for ::windows::core::IUnknown {
    fn from(value: IAttach) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAttach> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAttach) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAttach> for ::windows::core::IUnknown {
    fn from(value: &IAttach) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAttach> for IMAPIProp {
    fn from(value: IAttach) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAttach> for &'a IMAPIProp {
    fn from(value: &'a IAttach) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAttach> for IMAPIProp {
    fn from(value: &IAttach) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAttach {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAttach {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAttach {}
impl ::core::fmt::Debug for IAttach {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAttach").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAttach {
    type Vtable = IAttach_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttach_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IDistList(::windows::core::IUnknown);
impl IDistList {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetContentsTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetHierarchyTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSearchCriteria)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSearchCriteria)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ulcreateflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPIProp>(result__)
    }
    pub unsafe fn CopyEntries<'a, P0>(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpentries), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpentries), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResolveNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpadrlist), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<_flaglist>(result__)
    }
}
impl ::core::convert::From<IDistList> for ::windows::core::IUnknown {
    fn from(value: IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDistList> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDistList> for ::windows::core::IUnknown {
    fn from(value: &IDistList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDistList> for IMAPIProp {
    fn from(value: IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDistList> for &'a IMAPIProp {
    fn from(value: &'a IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDistList> for IMAPIProp {
    fn from(value: &IDistList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDistList> for IMAPIContainer {
    fn from(value: IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDistList> for &'a IMAPIContainer {
    fn from(value: &'a IDistList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDistList> for IMAPIContainer {
    fn from(value: &IDistList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDistList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDistList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDistList {}
impl ::core::fmt::Debug for IDistList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDistList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDistList {
    type Vtable = IDistList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDistList_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub DeleteEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ResolveNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ResolveNames: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIAdviseSink(::windows::core::IUnknown);
impl IMAPIAdviseSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
        (::windows::core::Interface::vtable(self).OnNotify)(::windows::core::Interface::as_raw(self), cnotif, ::core::mem::transmute(lpnotifications))
    }
}
impl ::core::convert::From<IMAPIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IMAPIAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIAdviseSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IMAPIAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIAdviseSink {}
impl ::core::fmt::Debug for IMAPIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIAdviseSink {
    type Vtable = IMAPIAdviseSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIAdviseSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnNotify: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIContainer(::windows::core::IUnknown);
impl IMAPIContainer {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetContentsTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHierarchyTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSearchCriteria)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSearchCriteria)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
}
impl ::core::convert::From<IMAPIContainer> for ::windows::core::IUnknown {
    fn from(value: IMAPIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIContainer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIContainer> for ::windows::core::IUnknown {
    fn from(value: &IMAPIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMAPIContainer> for IMAPIProp {
    fn from(value: IMAPIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIContainer> for &'a IMAPIProp {
    fn from(value: &'a IMAPIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIContainer> for IMAPIProp {
    fn from(value: &IMAPIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIContainer {}
impl ::core::fmt::Debug for IMAPIContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIContainer {
    type Vtable = IMAPIContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIContainer_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub GetContentsTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHierarchyTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetSearchCriteria: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetSearchCriteria: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSearchCriteria: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSearchCriteria: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIControl(::windows::core::IUnknown);
impl IMAPIControl {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut MAPIERROR>(result__)
    }
    pub unsafe fn Activate(&self, ulflags: u32, uluiparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), ulflags, uluiparam).ok()
    }
    pub unsafe fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpulstate)).ok()
    }
}
impl ::core::convert::From<IMAPIControl> for ::windows::core::IUnknown {
    fn from(value: IMAPIControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIControl> for ::windows::core::IUnknown {
    fn from(value: &IMAPIControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIControl {}
impl ::core::fmt::Debug for IMAPIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIControl {
    type Vtable = IMAPIControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, uluiparam: usize) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIFolder(::windows::core::IUnknown);
impl IMAPIFolder {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetContentsTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetContentsTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn GetHierarchyTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetHierarchyTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSearchCriteria)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), ::core::mem::transmute(lpcontainerlist), ulsearchflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSearchCriteria)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpprestriction), ::core::mem::transmute(lppcontainerlist), ::core::mem::transmute(lpulsearchstate)).ok()
    }
    pub unsafe fn CreateMessage(&self, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::core::option::Option<IMessage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lppmessage)).ok()
    }
    pub unsafe fn CopyMessages<'a, P0>(&self, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyMessages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpmsglist), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestfolder), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteMessages<'a, P0>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).DeleteMessages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpmsglist), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IMAPIFolder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateFolder)(::windows::core::Interface::as_raw(self), ulfoldertype, ::core::mem::transmute(lpszfoldername), ::core::mem::transmute(lpszfoldercomment), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPIFolder>(result__)
    }
    pub unsafe fn CopyFolder<'a, P0>(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyFolder)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestfolder), ::core::mem::transmute(lpsznewfoldername), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn DeleteFolder<'a, P0>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).DeleteFolder)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn SetReadFlags<'a, P0>(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).SetReadFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpmsglist), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMessageStatus)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SetMessageStatus)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ulnewstatus, ulnewstatusmask, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveContentsSort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpsortcriteria), ulflags).ok()
    }
    pub unsafe fn EmptyFolder<'a, P0>(&self, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).EmptyFolder)(::windows::core::Interface::as_raw(self), uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
}
impl ::core::convert::From<IMAPIFolder> for ::windows::core::IUnknown {
    fn from(value: IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIFolder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIFolder> for ::windows::core::IUnknown {
    fn from(value: &IMAPIFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMAPIFolder> for IMAPIProp {
    fn from(value: IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIFolder> for &'a IMAPIProp {
    fn from(value: &'a IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIFolder> for IMAPIProp {
    fn from(value: &IMAPIFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMAPIFolder> for IMAPIContainer {
    fn from(value: IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIFolder> for &'a IMAPIContainer {
    fn from(value: &'a IMAPIFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIFolder> for IMAPIContainer {
    fn from(value: &IMAPIFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIFolder {}
impl ::core::fmt::Debug for IMAPIFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIFolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIFolder {
    type Vtable = IMAPIFolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIFolder_Vtbl {
    pub base__: IMAPIContainer_Vtbl,
    pub CreateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub DeleteMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub CreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub SetReadFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub GetMessageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows::core::HRESULT,
    pub SetMessageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows::core::HRESULT,
    pub SaveContentsSort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT,
    pub EmptyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIProgress(::windows::core::IUnknown);
impl IMAPIProgress {
    pub unsafe fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Progress)(::windows::core::Interface::as_raw(self), ulvalue, ulcount, ultotal).ok()
    }
    pub unsafe fn GetFlags(&self, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulflags)).ok()
    }
    pub unsafe fn GetMax(&self, lpulmax: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMax)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulmax)).ok()
    }
    pub unsafe fn GetMin(&self, lpulmin: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMin)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulmin)).ok()
    }
    pub unsafe fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLimits)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulmin), ::core::mem::transmute(lpulmax), ::core::mem::transmute(lpulflags)).ok()
    }
}
impl ::core::convert::From<IMAPIProgress> for ::windows::core::IUnknown {
    fn from(value: IMAPIProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIProgress> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIProgress> for ::windows::core::IUnknown {
    fn from(value: &IMAPIProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIProgress {}
impl ::core::fmt::Debug for IMAPIProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIProgress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIProgress {
    type Vtable = IMAPIProgress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIProgress_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulmin: *mut u32) -> ::windows::core::HRESULT,
    pub SetLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIProp(::windows::core::IUnknown);
impl IMAPIProp {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
}
impl ::core::convert::From<IMAPIProp> for ::windows::core::IUnknown {
    fn from(value: IMAPIProp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIProp> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIProp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIProp> for ::windows::core::IUnknown {
    fn from(value: &IMAPIProp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIProp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIProp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIProp {}
impl ::core::fmt::Debug for IMAPIProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIProp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIProp {
    type Vtable = IMAPIProp_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIProp_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProps: usize,
    pub GetPropList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub OpenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetProps: usize,
    pub DeleteProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub CopyProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
    pub GetNamesFromIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT,
    pub GetIDsFromNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPIStatus(::windows::core::IUnknown);
impl IMAPIStatus {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn ValidateState(&self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ValidateState)(::windows::core::Interface::as_raw(self), uluiparam, ulflags).ok()
    }
    pub unsafe fn SettingsDialog(&self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SettingsDialog)(::windows::core::Interface::as_raw(self), uluiparam, ulflags).ok()
    }
    pub unsafe fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangePassword)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpoldpass), ::core::mem::transmute(lpnewpass), ulflags).ok()
    }
    pub unsafe fn FlushQueues(&self, uluiparam: usize, lptargettransport: &[ENTRYID], ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushQueues)(::windows::core::Interface::as_raw(self), uluiparam, lptargettransport.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lptargettransport)), ulflags).ok()
    }
}
impl ::core::convert::From<IMAPIStatus> for ::windows::core::IUnknown {
    fn from(value: IMAPIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIStatus> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIStatus> for ::windows::core::IUnknown {
    fn from(value: &IMAPIStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMAPIStatus> for IMAPIProp {
    fn from(value: IMAPIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPIStatus> for &'a IMAPIProp {
    fn from(value: &'a IMAPIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPIStatus> for IMAPIProp {
    fn from(value: &IMAPIStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPIStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPIStatus {}
impl ::core::fmt::Debug for IMAPIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPIStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPIStatus {
    type Vtable = IMAPIStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPIStatus_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub ValidateState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT,
    pub SettingsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::HRESULT,
    pub FlushQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMAPITable(::windows::core::IUnknown);
impl IMAPITable {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn Advise<'a, P0>(&self, uleventmask: u32, lpadvisesink: P0, lpulconnection: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIAdviseSink>>,
    {
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), uleventmask, lpadvisesink.into().abi(), ::core::mem::transmute(lpulconnection)).ok()
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpultablestatus), ::core::mem::transmute(lpultabletype)).ok()
    }
    pub unsafe fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags).ok()
    }
    pub unsafe fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryColumns)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpproptagarray)).ok()
    }
    pub unsafe fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRowCount)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpulcount)).ok()
    }
    pub unsafe fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SeekRow)(::windows::core::Interface::as_raw(self), bkorigin, lrowcount, ::core::mem::transmute(lplrowssought)).ok()
    }
    pub unsafe fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SeekRowApprox)(::windows::core::Interface::as_raw(self), ulnumerator, uldenominator).ok()
    }
    pub unsafe fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulrow), ::core::mem::transmute(lpulnumerator), ::core::mem::transmute(lpuldenominator)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindRow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), bkorigin, ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restrict)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lprestriction), ulflags).ok()
    }
    pub unsafe fn CreateBookmark(&self, lpbkposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateBookmark)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpbkposition)).ok()
    }
    pub unsafe fn FreeBookmark(&self, bkposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBookmark)(::windows::core::Interface::as_raw(self), bkposition).ok()
    }
    pub unsafe fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SortTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpsortcriteria), ulflags).ok()
    }
    pub unsafe fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QuerySortOrder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppsortcriteria)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryRows)(::windows::core::Interface::as_raw(self), lrowcount, ulflags, ::core::mem::transmute(lpprows)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExpandRow)(::windows::core::Interface::as_raw(self), cbinstancekey, ::core::mem::transmute(pbinstancekey), ulrowcount, ulflags, ::core::mem::transmute(lpprows), ::core::mem::transmute(lpulmorerows)).ok()
    }
    pub unsafe fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CollapseRow)(::windows::core::Interface::as_raw(self), cbinstancekey, ::core::mem::transmute(pbinstancekey), ulflags, ::core::mem::transmute(lpulrowcount)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForCompletion)(::windows::core::Interface::as_raw(self), ulflags, ultimeout, ::core::mem::transmute(lpultablestatus)).ok()
    }
    pub unsafe fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCollapseState)(::windows::core::Interface::as_raw(self), ulflags, cbinstancekey, ::core::mem::transmute(lpbinstancekey), ::core::mem::transmute(lpcbcollapsestate), ::core::mem::transmute(lppbcollapsestate)).ok()
    }
    pub unsafe fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCollapseState)(::windows::core::Interface::as_raw(self), ulflags, cbcollapsestate, ::core::mem::transmute(pbcollapsestate), ::core::mem::transmute(lpbklocation)).ok()
    }
}
impl ::core::convert::From<IMAPITable> for ::windows::core::IUnknown {
    fn from(value: IMAPITable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMAPITable> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMAPITable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMAPITable> for ::windows::core::IUnknown {
    fn from(value: &IMAPITable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMAPITable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMAPITable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMAPITable {}
impl ::core::fmt::Debug for IMAPITable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPITable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMAPITable {
    type Vtable = IMAPITable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMAPITable_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::HRESULT,
    pub QueryColumns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::HRESULT,
    pub SeekRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::HRESULT,
    pub SeekRowApprox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> ::windows::core::HRESULT,
    pub QueryPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FindRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FindRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Restrict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Restrict: usize,
    pub CreateBookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbkposition: *mut u32) -> ::windows::core::HRESULT,
    pub FreeBookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bkposition: u32) -> ::windows::core::HRESULT,
    pub SortTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT,
    pub QuerySortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryRows: usize,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExpandRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExpandRow: usize,
    pub CollapseRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555294i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555318i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_IMAGE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555320i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555319i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555350i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555352i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555351i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DATA_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555342i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIRECTORY_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555349i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIR_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555382i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DIR_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555366i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DISC_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555304i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_DUP_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555374i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_EMPTY_DISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555312i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555367i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555293i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555308i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555386i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555310i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555309i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_FSI_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555392i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555136i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555133i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555134i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555135i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGE_SIZE_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555360i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMAGE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555359i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555303i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555305i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_SEEK_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555306i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555298i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555307i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555301i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555341i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_DATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555387i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555391i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555376i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_VOLUME_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555388i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555328i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_ISO9660_LEVELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555343i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_ITEM_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555368i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_MULTISESSION_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555299i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_DIR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555383i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555384i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555381i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555300i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_OUTPUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555389i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555311i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_NO_UNIQUE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555373i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555296i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555390i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555375i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_MOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555326i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555336i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_READ_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555333i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555335i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555334i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_TOO_MANY_DIRS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555344i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555302i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555295i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1062555327i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11186527i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMailUser(::windows::core::IUnknown);
impl IMailUser {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
}
impl ::core::convert::From<IMailUser> for ::windows::core::IUnknown {
    fn from(value: IMailUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMailUser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMailUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMailUser> for ::windows::core::IUnknown {
    fn from(value: &IMailUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMailUser> for IMAPIProp {
    fn from(value: IMailUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMailUser> for &'a IMAPIProp {
    fn from(value: &'a IMailUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMailUser> for IMAPIProp {
    fn from(value: &IMailUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMailUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMailUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMailUser {}
impl ::core::fmt::Debug for IMailUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMailUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMailUser {
    type Vtable = IMailUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMailUser_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMessage(::windows::core::IUnknown);
impl IMessage {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn GetAttachmentTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAttachmentTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IAttach> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OpenAttach)(::windows::core::Interface::as_raw(self), ulattachmentnum, ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAttach>(result__)
    }
    pub unsafe fn CreateAttach(&self, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::core::option::Option<IAttach>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateAttach)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulattachmentnum), ::core::mem::transmute(lppattach)).ok()
    }
    pub unsafe fn DeleteAttach<'a, P0>(&self, ulattachmentnum: u32, uluiparam: usize, lpprogress: P0, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttach)(::windows::core::Interface::as_raw(self), ulattachmentnum, uluiparam, lpprogress.into().abi(), ulflags).ok()
    }
    pub unsafe fn GetRecipientTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecipientTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyRecipients)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpmods)).ok()
    }
    pub unsafe fn SubmitMessage(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SubmitMessage)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn SetReadFlag(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReadFlag)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
}
impl ::core::convert::From<IMessage> for ::windows::core::IUnknown {
    fn from(value: IMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMessage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessage> for ::windows::core::IUnknown {
    fn from(value: &IMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMessage> for IMAPIProp {
    fn from(value: IMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMessage> for &'a IMAPIProp {
    fn from(value: &'a IMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessage> for IMAPIProp {
    fn from(value: &IMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessage {}
impl ::core::fmt::Debug for IMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessage {
    type Vtable = IMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessage_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub GetAttachmentTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub GetRecipientTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ModifyRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ModifyRecipients: usize,
    pub SubmitMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub SetReadFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IMsgStore(::windows::core::IUnknown);
impl IMsgStore {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn Advise<'a, P0>(&self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIAdviseSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), uleventmask, lpadvisesink.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, ulconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), ulconnection).ok()
    }
    pub unsafe fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CompareEntryIDs)(::windows::core::Interface::as_raw(self), cbentryid1, ::core::mem::transmute(lpentryid1), cbentryid2, ::core::mem::transmute(lpentryid2), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenEntry)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(lpulobjtype), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn SetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiveFolder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpszmessageclass), ulflags, cbentryid, ::core::mem::transmute(lpentryid)).ok()
    }
    pub unsafe fn GetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReceiveFolder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpszmessageclass), ulflags, ::core::mem::transmute(lpcbentryid), ::core::mem::transmute(lppentryid), ::core::mem::transmute(lppszexplicitclass)).ok()
    }
    pub unsafe fn GetReceiveFolderTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReceiveFolderTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn StoreLogoff(&self, lpulflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StoreLogoff)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpulflags)).ok()
    }
    pub unsafe fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortSubmit)(::windows::core::Interface::as_raw(self), cbentryid, ::core::mem::transmute(lpentryid), ulflags).ok()
    }
    pub unsafe fn GetOutgoingQueue(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutgoingQueue)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    pub unsafe fn SetLockState<'a, P0>(&self, lpmessage: P0, ullockstate: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMessage>>,
    {
        (::windows::core::Interface::vtable(self).SetLockState)(::windows::core::Interface::as_raw(self), lpmessage.into().abi(), ullockstate).ok()
    }
    pub unsafe fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinishedMsg)(::windows::core::Interface::as_raw(self), ulflags, cbentryid, ::core::mem::transmute(lpentryid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyNewMail)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpnotification)).ok()
    }
}
impl ::core::convert::From<IMsgStore> for ::windows::core::IUnknown {
    fn from(value: IMsgStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMsgStore> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsgStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMsgStore> for ::windows::core::IUnknown {
    fn from(value: &IMsgStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IMsgStore> for IMAPIProp {
    fn from(value: IMsgStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMsgStore> for &'a IMAPIProp {
    fn from(value: &'a IMsgStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMsgStore> for IMAPIProp {
    fn from(value: &IMsgStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMsgStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMsgStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMsgStore {}
impl ::core::fmt::Debug for IMsgStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsgStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMsgStore {
    type Vtable = IMsgStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsgStore_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT,
    pub CompareEntryIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT,
    pub OpenEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReceiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT,
    pub GetReceiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::HRESULT,
    pub GetReceiveFolderTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StoreLogoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT,
    pub AbortSubmit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT,
    pub GetOutgoingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLockState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmessage: *mut ::core::ffi::c_void, ullockstate: u32) -> ::windows::core::HRESULT,
    pub FinishedMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyNewMail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnotification: *const NOTIFICATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyNewMail: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IProfSect(::windows::core::IUnknown);
impl IProfSect {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
}
impl ::core::convert::From<IProfSect> for ::windows::core::IUnknown {
    fn from(value: IProfSect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProfSect> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProfSect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProfSect> for ::windows::core::IUnknown {
    fn from(value: &IProfSect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IProfSect> for IMAPIProp {
    fn from(value: IProfSect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProfSect> for &'a IMAPIProp {
    fn from(value: &'a IProfSect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProfSect> for IMAPIProp {
    fn from(value: &IProfSect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IProfSect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProfSect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProfSect {}
impl ::core::fmt::Debug for IProfSect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProfSect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProfSect {
    type Vtable = IProfSect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProfSect_Vtbl {
    pub base__: IMAPIProp_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IPropData(::windows::core::IUnknown);
impl IPropData {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn SaveChanges(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveChanges)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ulflags, ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lppproparray)).ok()
    }
    pub unsafe fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropList)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lppproptagarray)).ok()
    }
    pub unsafe fn OpenProperty(&self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OpenProperty)(::windows::core::Interface::as_raw(self), ulproptag, ::core::mem::transmute(lpiid), ulinterfaceoptions, ulflags, ::core::mem::transmute(lppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProps)(::windows::core::Interface::as_raw(self), cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyTo<'a, P0>(&self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), ciidexclude, ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(lpexcludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn CopyProps<'a, P0>(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: P0, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMAPIProgress>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpincludeprops), uluiparam, lpprogress.into().abi(), ::core::mem::transmute(lpinterface), ::core::mem::transmute(lpdestobj), ulflags, ::core::mem::transmute(lppproblems)).ok()
    }
    pub unsafe fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNamesFromIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptags), ::core::mem::transmute(lppropsetguid), ulflags, ::core::mem::transmute(lpcpropnames), ::core::mem::transmute(lppppropnames)).ok()
    }
    pub unsafe fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIDsFromNames)(::windows::core::Interface::as_raw(self), cpropnames, ::core::mem::transmute(lpppropnames), ulflags, ::core::mem::transmute(lppproptags)).ok()
    }
    pub unsafe fn HrSetObjAccess(&self, ulaccess: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrSetObjAccess)(::windows::core::Interface::as_raw(self), ulaccess).ok()
    }
    pub unsafe fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrSetPropAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(rgulaccess)).ok()
    }
    pub unsafe fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrGetPropAccess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptagarray), ::core::mem::transmute(lprgulaccess)).ok()
    }
    pub unsafe fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrAddObjProps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lppproptagarray), ::core::mem::transmute(lprgulaccess)).ok()
    }
}
impl ::core::convert::From<IPropData> for ::windows::core::IUnknown {
    fn from(value: IPropData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropData> for ::windows::core::IUnknown {
    fn from(value: &IPropData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPropData> for IMAPIProp {
    fn from(value: IPropData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropData> for &'a IMAPIProp {
    fn from(value: &'a IPropData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropData> for IMAPIProp {
    fn from(value: &IPropData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropData {}
impl ::core::fmt::Debug for IPropData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropData {
    type Vtable = IPropData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropData_Vtbl {
    pub base__: IMAPIProp_Vtbl,
    pub HrSetObjAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulaccess: u32) -> ::windows::core::HRESULT,
    pub HrSetPropAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::HRESULT,
    pub HrGetPropAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::HRESULT,
    pub HrAddObjProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IProviderAdmin(::windows::core::IUnknown);
impl IProviderAdmin {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut MAPIERROR>(result__)
    }
    pub unsafe fn GetProviderTable(&self, ulflags: u32) -> ::windows::core::Result<IMAPITable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderTable)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPITable>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateProvider(&self, lpszprovider: *const i8, lpprops: &[SPropValue], uluiparam: usize, ulflags: u32) -> ::windows::core::Result<MAPIUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpszprovider), lpprops.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lpprops)), uluiparam, ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MAPIUID>(result__)
    }
    pub unsafe fn DeleteProvider(&self, lpuid: *const MAPIUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpuid)).ok()
    }
    pub unsafe fn OpenProfileSection(&self, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IProfSect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OpenProfileSection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpuid), ::core::mem::transmute(lpinterface), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProfSect>(result__)
    }
}
impl ::core::convert::From<IProviderAdmin> for ::windows::core::IUnknown {
    fn from(value: IProviderAdmin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProviderAdmin> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProviderAdmin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderAdmin> for ::windows::core::IUnknown {
    fn from(value: &IProviderAdmin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IProviderAdmin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderAdmin {}
impl ::core::fmt::Debug for IProviderAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderAdmin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProviderAdmin {
    type Vtable = IProviderAdmin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderAdmin_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub GetProviderTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateProvider: usize,
    pub DeleteProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID) -> ::windows::core::HRESULT,
    pub OpenProfileSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppprofsect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct ITableData(::windows::core::IUnknown);
impl ITableData {
    pub unsafe fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut ::core::option::Option<IMAPITable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrGetView)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpssortorderset), ::core::mem::transmute(lpfcallerrelease), ulcallerdata, ::core::mem::transmute(lppmapitable)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrModifyRow(&self, param0: *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrModifyRow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrDeleteRow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpspropvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrQueryRow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpspropvalue), ::core::mem::transmute(lppsrow), ::core::mem::transmute(lpulirow)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrEnumRow)(::windows::core::Interface::as_raw(self), ulrownumber, ::core::mem::transmute(lppsrow)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrNotify)(::windows::core::Interface::as_raw(self), ulflags, cvalues, ::core::mem::transmute(lpspropvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrInsertRow)(::windows::core::Interface::as_raw(self), ulirow, ::core::mem::transmute(lpsrow)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrModifyRows)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lpsrowset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HrDeleteRows)(::windows::core::Interface::as_raw(self), ulflags, ::core::mem::transmute(lprowsettodelete), ::core::mem::transmute(crowsdeleted)).ok()
    }
}
impl ::core::convert::From<ITableData> for ::windows::core::IUnknown {
    fn from(value: ITableData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITableData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITableData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITableData> for ::windows::core::IUnknown {
    fn from(value: &ITableData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITableData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITableData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableData {}
impl ::core::fmt::Debug for ITableData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITableData {
    type Vtable = ITableData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableData_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub HrGetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut *mut ::core::ffi::c_void, ulcallerdata: u32, lppmapitable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrModifyRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrModifyRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrDeleteRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrDeleteRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrQueryRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrQueryRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrEnumRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrEnumRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrNotify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrInsertRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrInsertRow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrModifyRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrModifyRows: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub HrDeleteRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    HrDeleteRows: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IWABExtInit(::windows::core::IUnknown);
impl IWABExtInit {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpwabextdisplay)).ok()
    }
}
impl ::core::convert::From<IWABExtInit> for ::windows::core::IUnknown {
    fn from(value: IWABExtInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWABExtInit> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWABExtInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWABExtInit> for ::windows::core::IUnknown {
    fn from(value: &IWABExtInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWABExtInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWABExtInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWABExtInit {}
impl ::core::fmt::Debug for IWABExtInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWABExtInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWABExtInit {
    type Vtable = IWABExtInit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea22ebf0_87a4_11d1_9acf_00a0c91f9c8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABExtInit_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IWABOBJECT_(::windows::core::IUnknown);
impl IWABOBJECT_ {
    pub unsafe fn QueryInterface(&self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn AddRef(&self) -> u32 {
        (::windows::core::Interface::vtable(self).AddRef)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Release(&self) -> u32 {
        (::windows::core::Interface::vtable(self).Release)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateBuffer)(::windows::core::Interface::as_raw(self), cbsize, ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateMore)(::windows::core::Interface::as_raw(self), cbsize, ::core::mem::transmute(lpobject), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpbuffer)).ok()
    }
    pub unsafe fn Backup<'a, P0>(&self, lpfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).Backup)(::windows::core::Interface::as_raw(self), lpfilename.into()).ok()
    }
    pub unsafe fn Import<'a, P0>(&self, lpwip: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), lpwip.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Find<'a, P0, P1>(&self, lpiab: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Find)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardDisplay<'a, P0, P1, P2>(&self, lpiab: P0, hwnd: P1, lpszfilename: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).VCardDisplay)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into(), lpszfilename.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LDAPUrl<'a, P0, P1, P2>(&self, lpiab: P0, hwnd: P1, ulflags: u32, lpszurl: P2) -> ::windows::core::Result<IMailUser>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::PCSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LDAPUrl)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into(), ulflags, lpszurl.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMailUser>(result__)
    }
    pub unsafe fn VCardCreate<'a, P0, P1, P2>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1, lpmailuser: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IMailUser>>,
    {
        (::windows::core::Interface::vtable(self).VCardCreate)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, lpszvcard.into(), lpmailuser.into().abi()).ok()
    }
    pub unsafe fn VCardRetrieve<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1) -> ::windows::core::Result<IMailUser>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VCardRetrieve)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, lpszvcard.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMailUser>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMe<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).GetMe)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, ::core::mem::transmute(lpdwaction), ::core::mem::transmute(lpsbeid), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMe<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, sbeid: SBinary, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetMe)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, ::core::mem::transmute(sbeid), hwnd.into()).ok()
    }
}
impl ::core::clone::Clone for IWABOBJECT_ {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWABOBJECT_ {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWABOBJECT_ {}
impl ::core::fmt::Debug for IWABOBJECT_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWABOBJECT_").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWABOBJECT_ {
    type Vtable = IWABOBJECT__Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABOBJECT__Vtbl {
    pub QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateMore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpwip: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Find: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Find: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VCardDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VCardDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LDAPUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LDAPUrl: usize,
    pub VCardCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lpmailuser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VCardRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMe: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AddRef_METHOD = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AllocateBuffer_METHOD = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_AllocateMore_METHOD = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Backup_METHOD = ::core::option::Option<unsafe extern "system" fn(lpfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Find_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_FreeBuffer_METHOD = ::core::option::Option<unsafe extern "system" fn(lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_GetLastError_METHOD = ::core::option::Option<unsafe extern "system" fn(hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_GetMe_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Import_METHOD = ::core::option::Option<unsafe extern "system" fn(lpwip: ::windows::core::PCSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_LDAPUrl_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows::core::PCSTR, lppmailuser: *mut ::core::option::Option<IMailUser>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_QueryInterface_METHOD = ::core::option::Option<unsafe extern "system" fn(riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_Release_METHOD = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_SetMe_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_VCardCreate_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lpmailuser: ::core::option::Option<IMailUser>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardDisplay_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type IWABOBJECT_VCardRetrieve_METHOD = ::core::option::Option<unsafe extern "system" fn(lpiab: ::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lppmailuser: *mut ::core::option::Option<IMailUser>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[repr(transparent)]
pub struct IWABObject(::windows::core::IUnknown);
impl IWABObject {
    pub unsafe fn GetLastError(&self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastError)(::windows::core::Interface::as_raw(self), hresult, ulflags, ::core::mem::transmute(lppmapierror)).ok()
    }
    pub unsafe fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateBuffer)(::windows::core::Interface::as_raw(self), cbsize, ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn AllocateMore(&self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateMore)(::windows::core::Interface::as_raw(self), cbsize, ::core::mem::transmute(lpobject), ::core::mem::transmute(lppbuffer)).ok()
    }
    pub unsafe fn FreeBuffer(&self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpbuffer)).ok()
    }
    pub unsafe fn Backup<'a, P0>(&self, lpfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).Backup)(::windows::core::Interface::as_raw(self), lpfilename.into()).ok()
    }
    pub unsafe fn Import<'a, P0>(&self, lpwip: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), lpwip.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Find<'a, P0, P1>(&self, lpiab: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Find)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VCardDisplay<'a, P0, P1, P2>(&self, lpiab: P0, hwnd: P1, lpszfilename: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).VCardDisplay)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into(), lpszfilename.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LDAPUrl<'a, P0, P1, P2>(&self, lpiab: P0, hwnd: P1, ulflags: u32, lpszurl: P2) -> ::windows::core::Result<IMailUser>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::PCSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LDAPUrl)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), hwnd.into(), ulflags, lpszurl.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMailUser>(result__)
    }
    pub unsafe fn VCardCreate<'a, P0, P1, P2>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1, lpmailuser: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, IMailUser>>,
    {
        (::windows::core::Interface::vtable(self).VCardCreate)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, lpszvcard.into(), lpmailuser.into().abi()).ok()
    }
    pub unsafe fn VCardRetrieve<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, lpszvcard: P1) -> ::windows::core::Result<IMailUser>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<::windows::core::PCSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VCardRetrieve)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, lpszvcard.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMailUser>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMe<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).GetMe)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, ::core::mem::transmute(lpdwaction), ::core::mem::transmute(lpsbeid), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMe<'a, P0, P1>(&self, lpiab: P0, ulflags: u32, sbeid: SBinary, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAddrBook>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetMe)(::windows::core::Interface::as_raw(self), lpiab.into().abi(), ulflags, ::core::mem::transmute(sbeid), hwnd.into()).ok()
    }
}
impl ::core::convert::From<IWABObject> for ::windows::core::IUnknown {
    fn from(value: IWABObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWABObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWABObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWABObject> for ::windows::core::IUnknown {
    fn from(value: &IWABObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWABObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWABObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWABObject {}
impl ::core::fmt::Debug for IWABObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWABObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWABObject {
    type Vtable = IWABObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWABObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateMore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpwip: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Find: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Find: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VCardDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VCardDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LDAPUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LDAPUrl: usize,
    pub VCardCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lpmailuser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VCardRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows::core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMe: usize,
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPALLOCATEBUFFER = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPALLOCATEMORE = ::core::option::Option<unsafe extern "system" fn(cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPCREATECONVERSATIONINDEX = ::core::option::Option<unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPDISPATCHNOTIFICATIONS = ::core::option::Option<unsafe extern "system" fn(ulflags: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNABSDI = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFNBUTTON = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFNDISMISS = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpbuffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNOTIFCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpvcontext: *mut ::core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = ::core::option::Option<unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABALLOCATEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpwabobject: ::core::option::Option<IWABObject>, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABALLOCATEMORE = ::core::option::Option<unsafe extern "system" fn(lpwabobject: ::core::option::Option<IWABObject>, cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub type LPWABFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(lpwabobject: ::core::option::Option<IWABObject>, lpbuffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPEN = ::core::option::Option<unsafe extern "system" fn(lppadrbook: *mut ::core::option::Option<IAddrBook>, lppwabobject: *mut ::core::option::Option<IWABObject>, lpwp: *mut WAB_PARAM, reserved2: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPENEX = ::core::option::Option<unsafe extern "system" fn(lppadrbook: *mut ::core::option::Option<IAddrBook>, lppwabobject: *mut ::core::option::Option<IWABObject>, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32;
    }
    LPropCompareProp(::core::mem::transmute(lpspropvaluea), ::core::mem::transmute(lpspropvalueb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue;
    }
    LpValFindProp(ulproptag, cvalues, ::core::mem::transmute(lpproparray))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn MAPIDeinitIdle() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MAPIDeinitIdle();
    }
    MAPIDeinitIdle()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
}
impl ::core::marker::Copy for MAPIERROR {}
impl ::core::clone::Clone for MAPIERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPIERROR").field("ulVersion", &self.ulVersion).field("lpszError", &self.lpszError).field("lpszComponent", &self.lpszComponent).field("ulLowLevelError", &self.ulLowLevelError).field("ulContext", &self.ulContext).finish()
    }
}
unsafe impl ::windows::core::Abi for MAPIERROR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAPIERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAPIERROR>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAPIERROR {}
impl ::core::default::Default for MAPIERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc>;
    }
    MAPIGetDefaultMalloc()
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32;
    }
    MAPIInitIdle(::core::mem::transmute(lpvreserved))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPINAMEID {
    pub lpguid: *mut ::windows::core::GUID,
    pub ulKind: u32,
    pub Kind: MAPINAMEID_0,
}
impl ::core::marker::Copy for MAPINAMEID {}
impl ::core::clone::Clone for MAPINAMEID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAPINAMEID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAPINAMEID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAPINAMEID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAPINAMEID {}
impl ::core::default::Default for MAPINAMEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub union MAPINAMEID_0 {
    pub lID: i32,
    pub lpwstrName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MAPINAMEID_0 {}
impl ::core::clone::Clone for MAPINAMEID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAPINAMEID_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAPINAMEID_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAPINAMEID_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAPINAMEID_0 {}
impl ::core::default::Default for MAPINAMEID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
impl ::core::marker::Copy for MAPIUID {}
impl ::core::clone::Clone for MAPIUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAPIUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAPIUID").field("ab", &self.ab).finish()
    }
}
unsafe impl ::windows::core::Abi for MAPIUID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAPIUID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAPIUID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAPIUID {}
impl ::core::default::Default for MAPIUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_COMPOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_DIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_ERROR_VERSION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOTRECIP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOTRESERVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_NOW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_P1: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_SHORTTERM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_THISSESSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MAPI_USE_DEFAULT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MNID_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MNID_STRING: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for MTSID {}
impl ::core::clone::Clone for MTSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MTSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MTSID").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
unsafe impl ::windows::core::Abi for MTSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MTSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MTSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MTSID {}
impl ::core::default::Default for MTSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MV_FLAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const MV_INSTANCE: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
}
impl ::core::marker::Copy for NEWMAIL_NOTIFICATION {}
impl ::core::clone::Clone for NEWMAIL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NEWMAIL_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWMAIL_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("cbParentID", &self.cbParentID).field("lpParentID", &self.lpParentID).field("ulFlags", &self.ulFlags).field("lpszMessageClass", &self.lpszMessageClass).field("ulMessageFlags", &self.ulMessageFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for NEWMAIL_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NEWMAIL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NEWMAIL_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for NEWMAIL_NOTIFICATION {}
impl ::core::default::Default for NEWMAIL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct NOTIFICATION {
    pub ulEventType: u32,
    pub ulAlignPad: u32,
    pub info: NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for NOTIFICATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFICATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
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
impl ::core::marker::Copy for NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for NOTIFICATION_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for NOTIFICATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFICATION_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
impl ::core::marker::Copy for NOTIFKEY {}
impl ::core::clone::Clone for NOTIFKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NOTIFKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFKEY").field("cb", &self.cb).field("ab", &self.ab).finish()
    }
}
unsafe impl ::windows::core::Abi for NOTIFKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NOTIFKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for NOTIFKEY {}
impl ::core::default::Default for NOTIFKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
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
impl ::core::marker::Copy for OBJECT_NOTIFICATION {}
impl ::core::clone::Clone for OBJECT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("ulObjType", &self.ulObjType).field("cbParentID", &self.cbParentID).field("lpParentID", &self.lpParentID).field("cbOldID", &self.cbOldID).field("lpOldID", &self.lpOldID).field("cbOldParentID", &self.cbOldParentID).field("lpOldParentID", &self.lpOldParentID).field("lpPropTagArray", &self.lpPropTagArray).finish()
    }
}
unsafe impl ::windows::core::Abi for OBJECT_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECT_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECT_NOTIFICATION {}
impl ::core::default::Default for OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const OPENSTREAMONFILE: &str = "OpenStreamOnFile";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8) -> ::windows::core::Result<super::Com::IStream> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenStreamOnFile(lpallocatebuffer: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OpenStreamOnFile(::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpfreebuffer), ulflags, ::core::mem::transmute(lpszfilename), ::core::mem::transmute(lpszprefix), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IStream>(result__)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNIDLE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRIHIGHEST: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRILOWEST: i32 = -32768i32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PRIUSER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
    }
    PpropFindProp(::core::mem::transmute(lpproparray), cvalues, ulproptag)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut ::core::ffi::c_void) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: *mut ::core::ffi::c_void, lpvobject: *mut ::core::ffi::c_void) -> i32;
    }
    PropCopyMore(::core::mem::transmute(lpspropvaluedest), ::core::mem::transmute(lpspropvaluesrc), ::core::mem::transmute(lpfallocmore), ::core::mem::transmute(lpvobject))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RTFSync<'a, P0>(lpmessage: P0, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IMessage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RTFSync(lpmessage: *mut ::core::ffi::c_void, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    RTFSync(lpmessage.into().abi(), ulflags, ::core::mem::transmute(lpfmessageupdated)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SAndRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SAndRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAndRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SAndRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SAndRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAndRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SAndRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAndRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
impl ::core::marker::Copy for SAppTimeArray {}
impl ::core::clone::Clone for SAppTimeArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAppTimeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAppTimeArray").field("cValues", &self.cValues).field("lpat", &self.lpat).finish()
    }
}
unsafe impl ::windows::core::Abi for SAppTimeArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAppTimeArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAppTimeArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAppTimeArray {}
impl ::core::default::Default for SAppTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
impl ::core::marker::Copy for SBinary {}
impl ::core::clone::Clone for SBinary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SBinary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBinary").field("cb", &self.cb).field("lpb", &self.lpb).finish()
    }
}
unsafe impl ::windows::core::Abi for SBinary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SBinary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SBinary>()) == 0 }
    }
}
impl ::core::cmp::Eq for SBinary {}
impl ::core::default::Default for SBinary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
impl ::core::marker::Copy for SBinaryArray {}
impl ::core::clone::Clone for SBinaryArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SBinaryArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBinaryArray").field("cValues", &self.cValues).field("lpbin", &self.lpbin).finish()
    }
}
unsafe impl ::windows::core::Abi for SBinaryArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SBinaryArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SBinaryArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SBinaryArray {}
impl ::core::default::Default for SBinaryArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
impl ::core::marker::Copy for SBitMaskRestriction {}
impl ::core::clone::Clone for SBitMaskRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SBitMaskRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SBitMaskRestriction").field("relBMR", &self.relBMR).field("ulPropTag", &self.ulPropTag).field("ulMask", &self.ulMask).finish()
    }
}
unsafe impl ::windows::core::Abi for SBitMaskRestriction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SBitMaskRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SBitMaskRestriction>()) == 0 }
    }
}
impl ::core::cmp::Eq for SBitMaskRestriction {}
impl ::core::default::Default for SBitMaskRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SCommentRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SCommentRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCommentRestriction").field("cValues", &self.cValues).field("lpRes", &self.lpRes).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SCommentRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SCommentRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCommentRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SCommentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SCommentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
impl ::core::marker::Copy for SComparePropsRestriction {}
impl ::core::clone::Clone for SComparePropsRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SComparePropsRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SComparePropsRestriction").field("relop", &self.relop).field("ulPropTag1", &self.ulPropTag1).field("ulPropTag2", &self.ulPropTag2).finish()
    }
}
unsafe impl ::windows::core::Abi for SComparePropsRestriction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SComparePropsRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SComparePropsRestriction>()) == 0 }
    }
}
impl ::core::cmp::Eq for SComparePropsRestriction {}
impl ::core::default::Default for SComparePropsRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SContentRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SContentRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SContentRestriction").field("ulFuzzyLevel", &self.ulFuzzyLevel).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SContentRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SContentRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SContentRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SContentRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SContentRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SCurrencyArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SCurrencyArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCurrencyArray").field("cValues", &self.cValues).field("lpcur", &self.lpcur).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for SCurrencyArray {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SCurrencyArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCurrencyArray>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SCurrencyArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SCurrencyArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SDateTimeArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SDateTimeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDateTimeArray").field("cValues", &self.cValues).field("lpft", &self.lpft).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SDateTimeArray {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SDateTimeArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SDateTimeArray>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SDateTimeArray {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SDateTimeArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
impl ::core::marker::Copy for SDoubleArray {}
impl ::core::clone::Clone for SDoubleArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SDoubleArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDoubleArray").field("cValues", &self.cValues).field("lpdbl", &self.lpdbl).finish()
    }
}
unsafe impl ::windows::core::Abi for SDoubleArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SDoubleArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SDoubleArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SDoubleArray {}
impl ::core::default::Default for SDoubleArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
impl ::core::marker::Copy for SExistRestriction {}
impl ::core::clone::Clone for SExistRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SExistRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SExistRestriction").field("ulReserved1", &self.ulReserved1).field("ulPropTag", &self.ulPropTag).field("ulReserved2", &self.ulReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for SExistRestriction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SExistRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SExistRestriction>()) == 0 }
    }
}
impl ::core::cmp::Eq for SExistRestriction {}
impl ::core::default::Default for SExistRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for SGuidArray {}
impl ::core::clone::Clone for SGuidArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SGuidArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SGuidArray").field("cValues", &self.cValues).field("lpguid", &self.lpguid).finish()
    }
}
unsafe impl ::windows::core::Abi for SGuidArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SGuidArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SGuidArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SGuidArray {}
impl ::core::default::Default for SGuidArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut ::windows::core::PSTR,
}
impl ::core::marker::Copy for SLPSTRArray {}
impl ::core::clone::Clone for SLPSTRArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLPSTRArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLPSTRArray").field("cValues", &self.cValues).field("lppszA", &self.lppszA).finish()
    }
}
unsafe impl ::windows::core::Abi for SLPSTRArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SLPSTRArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLPSTRArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SLPSTRArray {}
impl ::core::default::Default for SLPSTRArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
impl ::core::marker::Copy for SLargeIntegerArray {}
impl ::core::clone::Clone for SLargeIntegerArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLargeIntegerArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLargeIntegerArray").field("cValues", &self.cValues).field("lpli", &self.lpli).finish()
    }
}
unsafe impl ::windows::core::Abi for SLargeIntegerArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SLargeIntegerArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLargeIntegerArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SLargeIntegerArray {}
impl ::core::default::Default for SLargeIntegerArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
impl ::core::marker::Copy for SLongArray {}
impl ::core::clone::Clone for SLongArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLongArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLongArray").field("cValues", &self.cValues).field("lpl", &self.lpl).finish()
    }
}
unsafe impl ::windows::core::Abi for SLongArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SLongArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLongArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SLongArray {}
impl ::core::default::Default for SLongArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SNotRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SNotRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNotRestriction").field("ulReserved", &self.ulReserved).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SNotRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SNotRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SNotRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SNotRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SNotRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SOrRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SOrRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOrRestriction").field("cRes", &self.cRes).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SOrRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SOrRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOrRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SOrRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SOrRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl ::core::marker::Copy for SPropProblem {}
impl ::core::clone::Clone for SPropProblem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPropProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropProblem").field("ulIndex", &self.ulIndex).field("ulPropTag", &self.ulPropTag).field("scode", &self.scode).finish()
    }
}
unsafe impl ::windows::core::Abi for SPropProblem {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPropProblem {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropProblem>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPropProblem {}
impl ::core::default::Default for SPropProblem {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
impl ::core::marker::Copy for SPropProblemArray {}
impl ::core::clone::Clone for SPropProblemArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPropProblemArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropProblemArray").field("cProblem", &self.cProblem).field("aProblem", &self.aProblem).finish()
    }
}
unsafe impl ::windows::core::Abi for SPropProblemArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPropProblemArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropProblemArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPropProblemArray {}
impl ::core::default::Default for SPropProblemArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
}
impl ::core::marker::Copy for SPropTagArray {}
impl ::core::clone::Clone for SPropTagArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPropTagArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropTagArray").field("cValues", &self.cValues).field("aulPropTag", &self.aulPropTag).finish()
    }
}
unsafe impl ::windows::core::Abi for SPropTagArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPropTagArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropTagArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPropTagArray {}
impl ::core::default::Default for SPropTagArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropValue {
    pub ulPropTag: u32,
    pub dwAlignPad: u32,
    pub Value: _PV,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SPropValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SPropValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SPropValue {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SPropValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropValue>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SPropValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SPropertyRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SPropertyRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropertyRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("lpProp", &self.lpProp).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SPropertyRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SPropertyRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropertyRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SPropertyRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SPropertyRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
}
impl ::core::marker::Copy for SRealArray {}
impl ::core::clone::Clone for SRealArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SRealArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRealArray").field("cValues", &self.cValues).field("lpflt", &self.lpflt).finish()
    }
}
unsafe impl ::windows::core::Abi for SRealArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SRealArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SRealArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SRealArray {}
impl ::core::default::Default for SRealArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRestriction {
    pub rt: u32,
    pub res: SRestriction_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
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
impl ::core::marker::Copy for SRestriction_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRestriction_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRestriction_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRestriction_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SRestriction_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRestriction_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRestriction_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRow {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRow").field("ulAdrEntryPad", &self.ulAdrEntryPad).field("cValues", &self.cValues).field("lpProps", &self.lpProps).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRow {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRow {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SRow>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRow {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SRowSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SRowSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRowSet").field("cRows", &self.cRows).field("aRow", &self.aRow).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SRowSet {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SRowSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SRowSet>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SRowSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SRowSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
impl ::core::marker::Copy for SShortArray {}
impl ::core::clone::Clone for SShortArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SShortArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SShortArray").field("cValues", &self.cValues).field("lpi", &self.lpi).finish()
    }
}
unsafe impl ::windows::core::Abi for SShortArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SShortArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SShortArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SShortArray {}
impl ::core::default::Default for SShortArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
impl ::core::marker::Copy for SSizeRestriction {}
impl ::core::clone::Clone for SSizeRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SSizeRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSizeRestriction").field("relop", &self.relop).field("ulPropTag", &self.ulPropTag).field("cb", &self.cb).finish()
    }
}
unsafe impl ::windows::core::Abi for SSizeRestriction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SSizeRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSizeRestriction>()) == 0 }
    }
}
impl ::core::cmp::Eq for SSizeRestriction {}
impl ::core::default::Default for SSizeRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
impl ::core::marker::Copy for SSortOrder {}
impl ::core::clone::Clone for SSortOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SSortOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSortOrder").field("ulPropTag", &self.ulPropTag).field("ulOrder", &self.ulOrder).finish()
    }
}
unsafe impl ::windows::core::Abi for SSortOrder {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SSortOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSortOrder>()) == 0 }
    }
}
impl ::core::cmp::Eq for SSortOrder {}
impl ::core::default::Default for SSortOrder {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
impl ::core::marker::Copy for SSortOrderSet {}
impl ::core::clone::Clone for SSortOrderSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SSortOrderSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSortOrderSet").field("cSorts", &self.cSorts).field("cCategories", &self.cCategories).field("cExpanded", &self.cExpanded).field("aSort", &self.aSort).finish()
    }
}
unsafe impl ::windows::core::Abi for SSortOrderSet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SSortOrderSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSortOrderSet>()) == 0 }
    }
}
impl ::core::cmp::Eq for SSortOrderSet {}
impl ::core::default::Default for SSortOrderSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SSubRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SSubRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSubRestriction").field("ulSubObject", &self.ulSubObject).field("lpRes", &self.lpRes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SSubRestriction {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SSubRestriction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSubRestriction>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SSubRestriction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SSubRestriction {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for STATUS_OBJECT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for STATUS_OBJECT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATUS_OBJECT_NOTIFICATION").field("cbEntryID", &self.cbEntryID).field("lpEntryID", &self.lpEntryID).field("cValues", &self.cValues).field("lpPropVals", &self.lpPropVals).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for STATUS_OBJECT_NOTIFICATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for STATUS_OBJECT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATUS_OBJECT_NOTIFICATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for STATUS_OBJECT_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for STATUS_OBJECT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SWStringArray {}
impl ::core::clone::Clone for SWStringArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SWStringArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWStringArray").field("cValues", &self.cValues).field("lppszW", &self.lppszW).finish()
    }
}
unsafe impl ::windows::core::Abi for SWStringArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWStringArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWStringArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWStringArray {}
impl ::core::default::Default for SWStringArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_BOTHADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141126i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(11141632i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(11143688i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_ROTATIONADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141125i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_SPEEDADJUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(11141124i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(11141890i32);
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    }
    ScCopyNotifications(cnotification, ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpvdst), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    }
    ScCopyProps(cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpvdst), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32;
    }
    ScCountNotifications(cnotifications, ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32;
    }
    ScCountProps(cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
    }
    ScCreateConversationIndex(cbparent, ::core::mem::transmute(lpbparent), ::core::mem::transmute(lpcbconvindex), ::core::mem::transmute(lppbconvindex))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: *mut ::core::ffi::c_void, lppproparray: *mut *mut SPropValue) -> i32;
    }
    ScDupPropset(cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lppproparray))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn ScInitMapiUtil(ulflags: u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScInitMapiUtil(ulflags: u32) -> i32;
    }
    ScInitMapiUtil(ulflags)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn ScLocalPathFromUNC<'a, P0>(lpszunc: P0, lpszlocal: &[u8]) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScLocalPathFromUNC(lpszunc: ::windows::core::PCSTR, lpszlocal: ::windows::core::PCSTR, cchlocal: u32) -> i32;
    }
    ScLocalPathFromUNC(lpszunc.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpszlocal)), lpszlocal.len() as _)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    }
    ScRelocNotifications(cnotification, ::core::mem::transmute(lpnotifications), ::core::mem::transmute(lpvbaseold), ::core::mem::transmute(lpvbasenew), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    }
    ScRelocProps(cvalues, ::core::mem::transmute(lpproparray), ::core::mem::transmute(lpvbaseold), ::core::mem::transmute(lpvbasenew), ::core::mem::transmute(lpcb))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn ScUNCFromLocalPath<'a, P0>(lpszlocal: P0, lpszunc: &[u8]) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScUNCFromLocalPath(lpszlocal: ::windows::core::PCSTR, lpszunc: ::windows::core::PCSTR, cchunc: u32) -> i32;
    }
    ScUNCFromLocalPath(lpszlocal.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpszunc)), lpszunc.len() as _)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    }
    SzFindCh(::core::mem::transmute(lpsz), ch)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    }
    SzFindLastCh(::core::mem::transmute(lpsz), ch)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8;
    }
    SzFindSz(::core::mem::transmute(lpsz), ::core::mem::transmute(lpszkey))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_CHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ERROR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
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
impl ::core::marker::Copy for TABLE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for TABLE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for TABLE_NOTIFICATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for TABLE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TABLE_NOTIFICATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for TABLE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for TABLE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_RELOAD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_ADDED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_DELETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_SETCOL_DONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TABLE_SORT_DONE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const TAD_ALL_ROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn UFromSz(lpsz: *mut i8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UFromSz(lpsz: *mut i8) -> u32;
    }
    UFromSz(::core::mem::transmute(lpsz))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const UI_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32;
    }
    UlAddRef(::core::mem::transmute(lpunk))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32;
    }
    UlPropSize(::core::mem::transmute(lpspropvalue))
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32;
    }
    UlRelease(::core::mem::transmute(lpunk))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::clone::Clone for WABEXTDISPLAY {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            lpWABObject: self.lpWABObject.clone(),
            lpAdrBook: self.lpAdrBook.clone(),
            lpPropObj: self.lpPropObj.clone(),
            fReadOnly: self.fReadOnly,
            fDataChanged: self.fDataChanged,
            ulFlags: self.ulFlags,
            lpv: self.lpv,
            lpsz: self.lpsz,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABEXTDISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WABEXTDISPLAY").field("cbSize", &self.cbSize).field("lpWABObject", &self.lpWABObject).field("lpAdrBook", &self.lpAdrBook).field("lpPropObj", &self.lpPropObj).field("fReadOnly", &self.fReadOnly).field("fDataChanged", &self.fDataChanged).field("ulFlags", &self.ulFlags).field("lpv", &self.lpv).field("lpsz", &self.lpsz).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WABEXTDISPLAY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for WABEXTDISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: ::core::option::Option<IAddrBook>,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WABIMPORTPARAM {
    fn clone(&self) -> Self {
        Self { cbSize: self.cbSize, lpAdrBook: self.lpAdrBook.clone(), hWnd: self.hWnd, ulFlags: self.ulFlags, lpszFileName: self.lpszFileName }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WABIMPORTPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WABIMPORTPARAM").field("cbSize", &self.cbSize).field("lpAdrBook", &self.lpAdrBook).field("hWnd", &self.hWnd).field("ulFlags", &self.ulFlags).field("lpszFileName", &self.lpszFileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WABIMPORTPARAM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for WABIMPORTPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_ME_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DLL_NAME: &str = "WAB32.DLL";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_DLL_PATH_KEY: &str = "Software\\Microsoft\\WAB\\DLLPath";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: ::windows::core::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAB_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WAB_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAB_PARAM").field("cbSize", &self.cbSize).field("hwnd", &self.hwnd).field("szFileName", &self.szFileName).field("ulFlags", &self.ulFlags).field("guidPSExt", &self.guidPSExt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAB_PARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAB_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAB_PARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAB_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAB_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_VCARD_FILE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const WAB_VCARD_STREAM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WrapCompressedRTFStream<'a, P0>(lpcompressedrtfstream: P0, ulflags: u32) -> ::windows::core::Result<super::Com::IStream>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WrapCompressedRTFStream(lpcompressedrtfstream: *mut ::core::ffi::c_void, ulflags: u32, lpuncompressedrtfstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WrapCompressedRTFStream(lpcompressedrtfstream.into().abi(), ulflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IStream>(result__)
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
#[inline]
pub unsafe fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT;
    }
    WrapStoreEntryID(ulflags, ::core::mem::transmute(lpszdllname), cborigentry, ::core::mem::transmute(lporigentry), ::core::mem::transmute(lpcbwrappedentry), ::core::mem::transmute(lppwrappedentry)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
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
    pub lpszA: ::windows::core::PSTR,
    pub bin: SBinary,
    pub lpszW: ::windows::core::PWSTR,
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
impl ::core::marker::Copy for _PV {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _PV {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for _PV {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for _PV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_PV>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for _PV {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _PV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _WABACTIONITEM(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub struct _flaglist {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
impl ::core::marker::Copy for _flaglist {}
impl ::core::clone::Clone for _flaglist {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _flaglist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_flaglist").field("cFlags", &self.cFlags).field("ulFlag", &self.ulFlag).finish()
    }
}
unsafe impl ::windows::core::Abi for _flaglist {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _flaglist {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_flaglist>()) == 0 }
    }
}
impl ::core::cmp::Eq for _flaglist {}
impl ::core::default::Default for _flaglist {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const cchProfileNameMax: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const cchProfilePassMax: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const fMapiUnicode: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const hrSuccess: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szHrDispatchNotifications: &str = "HrDispatchNotifications";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szMAPINotificationMsg: &str = "MAPI Notify window message";
#[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
pub const szScCreateConversationIndex: &str = "ScCreateConversationIndex";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
