#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ACTIVEOBJECT_STRONG: u32 = 0u32;
pub const ACTIVEOBJECT_WEAK: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}
impl ARRAYDESC {}
impl ::std::default::Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ARRAYDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ARRAYDESC {}
unsafe impl ::windows::runtime::Abi for ARRAYDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::windows::runtime::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for BINDPTR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for BINDPTR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            );
        }
        ::std::mem::transmute(BSTR_UserFree(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree64(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            );
        }
        ::std::mem::transmute(BSTR_UserFree64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserMarshal(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::super::Foundation::BSTR,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserMarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserMarshal64(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::super::Foundation::BSTR,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal64(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserMarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserSize(
    param0: *const u32,
    param1: u32,
    param2: *const super::super::Foundation::BSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> u32;
        }
        ::std::mem::transmute(BSTR_UserSize(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserSize64(
    param0: *const u32,
    param1: u32,
    param2: *const super::super::Foundation::BSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize64(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> u32;
        }
        ::std::mem::transmute(BSTR_UserSize64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserUnmarshal(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::super::Foundation::BSTR,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserUnmarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BSTR_UserUnmarshal64(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::super::Foundation::BSTR,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal64(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserUnmarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn BstrFromVector(
    psa: *const SAFEARRAY,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BstrFromVector(
                psa: *const SAFEARRAY,
                pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        BstrFromVector(::std::mem::transmute(psa), &mut result__)
            .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct CALLCONV(pub i32);
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::std::convert::From<i32> for CALLCONV {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLCONV {
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
pub struct CHANGEKIND(pub i32);
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = CHANGEKIND(0i32);
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = CHANGEKIND(1i32);
pub const CHANGEKIND_SETNAMES: CHANGEKIND = CHANGEKIND(2i32);
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = CHANGEKIND(3i32);
pub const CHANGEKIND_GENERAL: CHANGEKIND = CHANGEKIND(4i32);
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = CHANGEKIND(5i32);
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = CHANGEKIND(6i32);
pub const CHANGEKIND_MAX: CHANGEKIND = CHANGEKIND(7i32);
impl ::std::convert::From<i32> for CHANGEKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHANGEKIND {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: ::std::option::Option<::windows::runtime::IUnknown>,
    pub pStorage: *mut ::std::ffi::c_void,
    pub flags: u32,
}
impl CLEANLOCALSTORAGE {}
impl ::std::default::Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLEANLOCALSTORAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLEANLOCALSTORAGE")
            .field("pInterface", &self.pInterface)
            .field("pStorage", &self.pStorage)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CLEANLOCALSTORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pInterface == other.pInterface
            && self.pStorage == other.pStorage
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for CLEANLOCALSTORAGE {}
unsafe impl ::windows::runtime::Abi for CLEANLOCALSTORAGE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for CUSTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CUSTDATA")
            .field("cCustData", &self.cCustData)
            .field("prgCustData", &self.prgCustData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cCustData == other.cCustData && self.prgCustData == other.prgCustData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for CUSTDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows::runtime::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for CUSTDATAITEM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ClearCustData(pcustdata: *mut CUSTDATA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearCustData(pcustdata: *mut CUSTDATA);
        }
        ::std::mem::transmute(ClearCustData(::std::mem::transmute(pcustdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateDispTypeInfo(
    pidata: *mut INTERFACEDATA,
    lcid: u32,
    pptinfo: *mut ::std::option::Option<ITypeInfo>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDispTypeInfo(
                pidata: *mut INTERFACEDATA,
                lcid: u32,
                pptinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        CreateDispTypeInfo(
            ::std::mem::transmute(pidata),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(pptinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateErrorInfo() -> ::windows::runtime::Result<ICreateErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateErrorInfo(
                pperrinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ICreateErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateErrorInfo(&mut result__).from_abi::<ICreateErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateStdDispatch<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    Param2: ::windows::runtime::IntoParam<'a, ITypeInfo>,
>(
    punkouter: Param0,
    pvthis: *mut ::std::ffi::c_void,
    ptinfo: Param2,
    ppunkstddisp: *mut ::std::option::Option<::windows::runtime::IUnknown>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdDispatch(
                punkouter: ::windows::runtime::RawPtr,
                pvthis: *mut ::std::ffi::c_void,
                ptinfo: ::windows::runtime::RawPtr,
                ppunkstddisp: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        CreateStdDispatch(
            punkouter.into_param().abi(),
            ::std::mem::transmute(pvthis),
            ptinfo.into_param().abi(),
            ::std::mem::transmute(ppunkstddisp),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateTypeLib<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    syskind: SYSKIND,
    szfile: Param1,
) -> ::windows::runtime::Result<ICreateTypeLib> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTypeLib(
                syskind: SYSKIND,
                szfile: super::super::Foundation::PWSTR,
                ppctlib: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ICreateTypeLib as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateTypeLib(
            ::std::mem::transmute(syskind),
            szfile.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ICreateTypeLib>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateTypeLib2<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    syskind: SYSKIND,
    szfile: Param1,
) -> ::windows::runtime::Result<ICreateTypeLib2> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTypeLib2(
                syskind: SYSKIND,
                szfile: super::super::Foundation::PWSTR,
                ppctlib: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ICreateTypeLib2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateTypeLib2(
            ::std::mem::transmute(syskind),
            szfile.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ICreateTypeLib2>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct DESCKIND(pub i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::std::convert::From<i32> for DESCKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DESCKIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
pub const DISPATCH_METHOD: u32 = 1u32;
pub const DISPATCH_PROPERTYGET: u32 = 2u32;
pub const DISPATCH_PROPERTYPUT: u32 = 4u32;
pub const DISPATCH_PROPERTYPUTREF: u32 = 8u32;
pub const DISPID_COLLECT: i32 = -8i32;
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
pub const DISPID_DESTRUCTOR: i32 = -7i32;
pub const DISPID_EVALUATE: i32 = -5i32;
pub const DISPID_NEWENUM: i32 = -4i32;
pub const DISPID_PROPERTYPUT: i32 = -3i32;
pub const DISPID_STARTENUM: i32 = -1i32;
pub const DISPID_THIS: i32 = -613i32;
pub const DISPID_UNKNOWN: i32 = -1i32;
pub const DISPID_VALUE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for DISPPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPPARAMS")
            .field("rgvarg", &self.rgvarg)
            .field("rgdispidNamedArgs", &self.rgdispidNamedArgs)
            .field("cArgs", &self.cArgs)
            .field("cNamedArgs", &self.cNamedArgs)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgvarg == other.rgvarg
            && self.rgdispidNamedArgs == other.rgdispidNamedArgs
            && self.cArgs == other.cArgs
            && self.cNamedArgs == other.cNamedArgs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DISPPARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DispCallFunc(
    pvinstance: *const ::std::ffi::c_void,
    ovft: usize,
    cc: CALLCONV,
    vtreturn: u16,
    cactuals: u32,
    prgvt: *const u16,
    prgpvarg: *const *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DispCallFunc(
                pvinstance: *const ::std::ffi::c_void,
                ovft: usize,
                cc: CALLCONV,
                vtreturn: u16,
                cactuals: u32,
                prgvt: *const u16,
                prgpvarg: *const *const VARIANT,
                pvargresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DispCallFunc(
            ::std::mem::transmute(pvinstance),
            ::std::mem::transmute(ovft),
            ::std::mem::transmute(cc),
            ::std::mem::transmute(vtreturn),
            ::std::mem::transmute(cactuals),
            ::std::mem::transmute(prgvt),
            ::std::mem::transmute(prgpvarg),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DispGetIDsOfNames<'a, Param0: ::windows::runtime::IntoParam<'a, ITypeInfo>>(
    ptinfo: Param0,
    rgsznames: *const super::super::Foundation::PWSTR,
    cnames: u32,
    rgdispid: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DispGetIDsOfNames(
                ptinfo: ::windows::runtime::RawPtr,
                rgsznames: *const super::super::Foundation::PWSTR,
                cnames: u32,
                rgdispid: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        DispGetIDsOfNames(
            ptinfo.into_param().abi(),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DispGetParam(
    pdispparams: *const DISPPARAMS,
    position: u32,
    vttarg: u16,
    pvarresult: *mut VARIANT,
    puargerr: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DispGetParam(
                pdispparams: *const DISPPARAMS,
                position: u32,
                vttarg: u16,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
                puargerr: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DispGetParam(
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(position),
            ::std::mem::transmute(vttarg),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DispInvoke<'a, Param1: ::windows::runtime::IntoParam<'a, ITypeInfo>>(
    _this: *mut ::std::ffi::c_void,
    ptinfo: Param1,
    dispidmember: i32,
    wflags: u16,
    pparams: *mut DISPPARAMS,
    pvarresult: *mut VARIANT,
    pexcepinfo: *mut EXCEPINFO,
    puargerr: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DispInvoke(
                _this: *mut ::std::ffi::c_void,
                ptinfo: ::windows::runtime::RawPtr,
                dispidmember: i32,
                wflags: u16,
                pparams: *mut DISPPARAMS,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
                pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
                puargerr: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DispInvoke(
            ::std::mem::transmute(_this),
            ptinfo.into_param().abi(),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32;
        }
        ::std::mem::transmute(DosDateTimeToVariantTime(
            ::std::mem::transmute(wdosdate),
            ::std::mem::transmute(wdostime),
            ::std::mem::transmute(pvtime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for ELEMDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for ELEMDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for ELEMDESC_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::std::ffi::c_void,
    pub pfnDeferredFillIn: ::std::option::Option<LPEXCEPFINO_DEFERRED_FILLIN>,
    pub scode: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EXCEPINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EXCEPINFO")
            .field("wCode", &self.wCode)
            .field("wReserved", &self.wReserved)
            .field("bstrSource", &self.bstrSource)
            .field("bstrDescription", &self.bstrDescription)
            .field("bstrHelpFile", &self.bstrHelpFile)
            .field("dwHelpContext", &self.dwHelpContext)
            .field("pvReserved", &self.pvReserved)
            .field("scode", &self.scode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EXCEPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wCode == other.wCode
            && self.wReserved == other.wReserved
            && self.bstrSource == other.bstrSource
            && self.bstrDescription == other.bstrDescription
            && self.bstrHelpFile == other.bstrHelpFile
            && self.dwHelpContext == other.dwHelpContext
            && self.pvReserved == other.pvReserved
            && self.pfnDeferredFillIn.map(|f| f as usize)
                == other.pfnDeferredFillIn.map(|f| f as usize)
            && self.scode == other.scode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EXCEPINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const FADF_AUTO: u32 = 1u32;
pub const FADF_BSTR: u32 = 256u32;
pub const FADF_DISPATCH: u32 = 1024u32;
pub const FADF_EMBEDDED: u32 = 4u32;
pub const FADF_FIXEDSIZE: u32 = 16u32;
pub const FADF_HAVEIID: u32 = 64u32;
pub const FADF_HAVEVARTYPE: u32 = 128u32;
pub const FADF_RECORD: u32 = 32u32;
pub const FADF_RESERVED: u32 = 61448u32;
pub const FADF_STATIC: u32 = 2u32;
pub const FADF_UNKNOWN: u32 = 512u32;
pub const FADF_VARIANT: u32 = 2048u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for FUNCDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for FUNCDESC {
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
pub struct FUNCFLAGS(pub i32);
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = FUNCFLAGS(1i32);
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = FUNCFLAGS(2i32);
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = FUNCFLAGS(4i32);
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = FUNCFLAGS(8i32);
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = FUNCFLAGS(16i32);
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = FUNCFLAGS(32i32);
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = FUNCFLAGS(64i32);
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = FUNCFLAGS(128i32);
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = FUNCFLAGS(256i32);
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = FUNCFLAGS(512i32);
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = FUNCFLAGS(1024i32);
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = FUNCFLAGS(2048i32);
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = FUNCFLAGS(4096i32);
impl ::std::convert::From<i32> for FUNCFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FUNCFLAGS {
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
pub struct FUNCKIND(pub i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::std::convert::From<i32> for FUNCKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FUNCKIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn GetActiveObject(
    rclsid: *const ::windows::runtime::GUID,
    pvreserved: *mut ::std::ffi::c_void,
    ppunk: *mut ::std::option::Option<::windows::runtime::IUnknown>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveObject(
                rclsid: *const ::windows::runtime::GUID,
                pvreserved: *mut ::std::ffi::c_void,
                ppunk: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        GetActiveObject(
            ::std::mem::transmute(rclsid),
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(ppunk),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetAltMonthNames(
    lcid: u32,
) -> ::windows::runtime::Result<*mut super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAltMonthNames(
                lcid: u32,
                prgp: *mut *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetAltMonthNames(::std::mem::transmute(lcid), &mut result__)
            .from_abi::<*mut super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::runtime::Result<IErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetErrorInfo(
                dwreserved: u32,
                pperrinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetErrorInfo(::std::mem::transmute(dwreserved), &mut result__)
            .from_abi::<IErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetRecordInfoFromGuids(
    rguidtypelib: *const ::windows::runtime::GUID,
    uvermajor: u32,
    uverminor: u32,
    lcid: u32,
    rguidtypeinfo: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<IRecordInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRecordInfoFromGuids(
                rguidtypelib: *const ::windows::runtime::GUID,
                uvermajor: u32,
                uverminor: u32,
                lcid: u32,
                rguidtypeinfo: *const ::windows::runtime::GUID,
                pprecinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRecordInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetRecordInfoFromGuids(
            ::std::mem::transmute(rguidtypelib),
            ::std::mem::transmute(uvermajor),
            ::std::mem::transmute(uverminor),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rguidtypeinfo),
            &mut result__,
        )
        .from_abi::<IRecordInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetRecordInfoFromTypeInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ITypeInfo>,
>(
    ptypeinfo: Param0,
) -> ::windows::runtime::Result<IRecordInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRecordInfoFromTypeInfo(
                ptypeinfo: ::windows::runtime::RawPtr,
                pprecinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRecordInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetRecordInfoFromTypeInfo(ptypeinfo.into_param().abi(), &mut result__)
            .from_abi::<IRecordInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree(param0: *const u32, param1: *const super::super::Foundation::HWND);
        }
        ::std::mem::transmute(HWND_UserFree(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree64(param0: *const u32, param1: *const super::super::Foundation::HWND);
        }
        ::std::mem::transmute(HWND_UserFree64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserMarshal(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::super::Foundation::HWND,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal(
                param0: *const u32,
                param1: *mut u8,
                param2: *const super::super::Foundation::HWND,
            ) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserMarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserMarshal64(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::super::Foundation::HWND,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal64(
                param0: *const u32,
                param1: *mut u8,
                param2: *const super::super::Foundation::HWND,
            ) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserMarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserSize(
    param0: *const u32,
    param1: u32,
    param2: *const super::super::Foundation::HWND,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize(
                param0: *const u32,
                param1: u32,
                param2: *const super::super::Foundation::HWND,
            ) -> u32;
        }
        ::std::mem::transmute(HWND_UserSize(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserSize64(
    param0: *const u32,
    param1: u32,
    param2: *const super::super::Foundation::HWND,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize64(
                param0: *const u32,
                param1: u32,
                param2: *const super::super::Foundation::HWND,
            ) -> u32;
        }
        ::std::mem::transmute(HWND_UserSize64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserUnmarshal(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::super::Foundation::HWND,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal(
                param0: *const u32,
                param1: *const u8,
                param2: *mut super::super::Foundation::HWND,
            ) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserUnmarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HWND_UserUnmarshal64(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::super::Foundation::HWND,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal64(
                param0: *const u32,
                param1: *const u8,
                param2: *mut super::super::Foundation::HWND,
            ) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserUnmarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICanHandleException(::windows::runtime::IUnknown);
impl ICanHandleException {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn CanHandleException(
        &self,
        pexcepinfo: *const EXCEPINFO,
        pvar: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(pvar),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICanHandleException {
    type Vtable = ICanHandleException_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3310980704,
        45831,
        4561,
        [178, 125, 0, 96, 8, 195, 251, 251],
    );
}
impl ::std::convert::From<ICanHandleException> for ::windows::runtime::IUnknown {
    fn from(value: ICanHandleException) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICanHandleException> for ::windows::runtime::IUnknown {
    fn from(value: &ICanHandleException) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICanHandleException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICanHandleException {
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
pub struct ICanHandleException_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pexcepinfo: *const ::std::mem::ManuallyDrop<EXCEPINFO>,
        pvar: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICreateErrorInfo(::windows::runtime::IUnknown);
impl ICreateErrorInfo {
    pub unsafe fn SetGUID(
        &self,
        rguid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rguid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szsource: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            szsource.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szdescription: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            szdescription.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHelpFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szhelpfile: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            szhelpfile.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateErrorInfo {
    type Vtable = ICreateErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        586167104,
        21629,
        4123,
        [142, 101, 8, 0, 43, 43, 209, 25],
    );
}
impl ::std::convert::From<ICreateErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: ICreateErrorInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateErrorInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateErrorInfo {
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
pub struct ICreateErrorInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rguid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szsource: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szdescription: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szhelpfile: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICreateTypeInfo(::windows::runtime::IUnknown);
impl ICreateTypeInfo {
    pub unsafe fn SetGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
        )
        .ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(utypeflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pstrdoc: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pstrdoc.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetVersion(
        &self,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wmajorvernum),
            ::std::mem::transmute(wminorvernum),
        )
        .ok()
    }
    pub unsafe fn AddRefTypeInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ITypeInfo>>(
        &self,
        ptinfo: Param0,
        phreftype: *const u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ptinfo.into_param().abi(),
            ::std::mem::transmute(phreftype),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn AddFuncDesc(
        &self,
        index: u32,
        pfuncdesc: *const FUNCDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pfuncdesc),
        )
        .ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(hreftype),
        )
        .ok()
    }
    pub unsafe fn SetImplTypeFlags(
        &self,
        index: u32,
        impltypeflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(impltypeflags),
        )
        .ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cbalignment),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSchema<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pstrschema: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pstrschema.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn AddVarDesc(
        &self,
        index: u32,
        pvardesc: *const VARDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pvardesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFuncAndParamNames(
        &self,
        index: u32,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVarName<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szname: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTypeDescAlias(
        &self,
        ptdescalias: *const TYPEDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptdescalias),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefineFuncAsDllEntry<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdllname: Param1,
        szprocname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdllname.into_param().abi(),
            szprocname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFuncDocString<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdocstring: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdocstring.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVarDocString<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdocstring: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdocstring.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetFuncHelpContext(
        &self,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetVarHelpContext(
        &self,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMops<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        index: u32,
        bstrmops: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            bstrmops.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTypeIdldesc(
        &self,
        pidldesc: *const IDLDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pidldesc),
        )
        .ok()
    }
    pub unsafe fn LayOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateTypeInfo {
    type Vtable = ICreateTypeInfo_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132101, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICreateTypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: ICreateTypeInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateTypeInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateTypeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateTypeInfo {
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
pub struct ICreateTypeInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        utypeflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstrdoc: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptinfo: ::windows::runtime::RawPtr,
        phreftype: *const u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pfuncdesc: *const FUNCDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        hreftype: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        impltypeflags: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cbalignment: u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstrschema: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pvardesc: *const VARDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptdescalias: *const TYPEDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdllname: super::super::Foundation::PWSTR,
        szprocname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdocstring: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdocstring: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        bstrmops: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pidldesc: *const IDLDESC,
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
pub struct ICreateTypeInfo2(::windows::runtime::IUnknown);
impl ICreateTypeInfo2 {
    pub unsafe fn SetGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
        )
        .ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(utypeflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pstrdoc: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pstrdoc.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetVersion(
        &self,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wmajorvernum),
            ::std::mem::transmute(wminorvernum),
        )
        .ok()
    }
    pub unsafe fn AddRefTypeInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ITypeInfo>>(
        &self,
        ptinfo: Param0,
        phreftype: *const u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ptinfo.into_param().abi(),
            ::std::mem::transmute(phreftype),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn AddFuncDesc(
        &self,
        index: u32,
        pfuncdesc: *const FUNCDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pfuncdesc),
        )
        .ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(hreftype),
        )
        .ok()
    }
    pub unsafe fn SetImplTypeFlags(
        &self,
        index: u32,
        impltypeflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(impltypeflags),
        )
        .ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cbalignment),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSchema<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pstrschema: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pstrschema.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn AddVarDesc(
        &self,
        index: u32,
        pvardesc: *const VARDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pvardesc),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFuncAndParamNames(
        &self,
        index: u32,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVarName<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szname: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTypeDescAlias(
        &self,
        ptdescalias: *const TYPEDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptdescalias),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefineFuncAsDllEntry<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdllname: Param1,
        szprocname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdllname.into_param().abi(),
            szprocname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFuncDocString<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdocstring: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdocstring.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVarDocString<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        index: u32,
        szdocstring: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            szdocstring.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetFuncHelpContext(
        &self,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetVarHelpContext(
        &self,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMops<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        index: u32,
        bstrmops: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            bstrmops.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTypeIdldesc(
        &self,
        pidldesc: *const IDLDESC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pidldesc),
        )
        .ok()
    }
    pub unsafe fn LayOut(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DeleteFuncDesc(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
        )
        .ok()
    }
    pub unsafe fn DeleteFuncDescByMemId(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
        )
        .ok()
    }
    pub unsafe fn DeleteVarDesc(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
        )
        .ok()
    }
    pub unsafe fn DeleteVarDescByMemId(&self, memid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
        )
        .ok()
    }
    pub unsafe fn DeleteImplType(&self, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetCustData(
        &self,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetFuncCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetParamCustData(
        &self,
        indexfunc: u32,
        indexparam: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(indexfunc),
            ::std::mem::transmute(indexparam),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetVarCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetImplTypeCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    pub unsafe fn SetHelpStringContext(
        &self,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpstringcontext),
        )
        .ok()
    }
    pub unsafe fn SetFuncHelpStringContext(
        &self,
        index: u32,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpstringcontext),
        )
        .ok()
    }
    pub unsafe fn SetVarHelpStringContext(
        &self,
        index: u32,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(dwhelpstringcontext),
        )
        .ok()
    }
    pub unsafe fn Invalidate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateTypeInfo2 {
    type Vtable = ICreateTypeInfo2_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132110, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICreateTypeInfo2> for ::windows::runtime::IUnknown {
    fn from(value: ICreateTypeInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateTypeInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateTypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateTypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICreateTypeInfo2> for ICreateTypeInfo {
    fn from(value: ICreateTypeInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeInfo2> for ICreateTypeInfo {
    fn from(value: &ICreateTypeInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICreateTypeInfo> for ICreateTypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICreateTypeInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICreateTypeInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICreateTypeInfo> for &ICreateTypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICreateTypeInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICreateTypeInfo>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        utypeflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstrdoc: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptinfo: ::windows::runtime::RawPtr,
        phreftype: *const u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pfuncdesc: *const FUNCDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        hreftype: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        impltypeflags: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cbalignment: u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstrschema: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pvardesc: *const VARDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptdescalias: *const TYPEDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdllname: super::super::Foundation::PWSTR,
        szprocname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdocstring: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        szdocstring: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        bstrmops: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pidldesc: *const IDLDESC,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        indexfunc: u32,
        indexparam: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
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
pub struct ICreateTypeLib(::windows::runtime::IUnknown);
impl ICreateTypeLib {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTypeInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
        tkind: TYPEKIND,
    ) -> ::windows::runtime::Result<ICreateTypeInfo> {
        let mut result__: <ICreateTypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
            ::std::mem::transmute(tkind),
            &mut result__,
        )
        .from_abi::<ICreateTypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetVersion(
        &self,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wmajorvernum),
            ::std::mem::transmute(wminorvernum),
        )
        .ok()
    }
    pub unsafe fn SetGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szdoc: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            szdoc.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHelpFileName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szhelpfilename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            szhelpfilename.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lcid),
        )
        .ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulibflags),
        )
        .ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateTypeLib {
    type Vtable = ICreateTypeLib_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132102, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICreateTypeLib> for ::windows::runtime::IUnknown {
    fn from(value: ICreateTypeLib) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeLib> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateTypeLib) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateTypeLib {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateTypeLib {
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
pub struct ICreateTypeLib_abi(
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
        szname: super::super::Foundation::PWSTR,
        tkind: TYPEKIND,
        ppctinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szdoc: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szhelpfilename: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lcid: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulibflags: u32,
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
pub struct ICreateTypeLib2(::windows::runtime::IUnknown);
impl ICreateTypeLib2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTypeInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
        tkind: TYPEKIND,
    ) -> ::windows::runtime::Result<ICreateTypeInfo> {
        let mut result__: <ICreateTypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
            ::std::mem::transmute(tkind),
            &mut result__,
        )
        .from_abi::<ICreateTypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetVersion(
        &self,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wmajorvernum),
            ::std::mem::transmute(wminorvernum),
        )
        .ok()
    }
    pub unsafe fn SetGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szdoc: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            szdoc.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHelpFileName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szhelpfilename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            szhelpfilename.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpcontext),
        )
        .ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lcid),
        )
        .ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulibflags),
        )
        .ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTypeInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn SetCustData(
        &self,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            ::std::mem::transmute(pvarval),
        )
        .ok()
    }
    pub unsafe fn SetHelpStringContext(
        &self,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwhelpstringcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHelpStringDll<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szfilename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            szfilename.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICreateTypeLib2 {
    type Vtable = ICreateTypeLib2_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132111, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICreateTypeLib2> for ::windows::runtime::IUnknown {
    fn from(value: ICreateTypeLib2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeLib2> for ::windows::runtime::IUnknown {
    fn from(value: &ICreateTypeLib2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICreateTypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICreateTypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICreateTypeLib2> for ICreateTypeLib {
    fn from(value: ICreateTypeLib2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICreateTypeLib2> for ICreateTypeLib {
    fn from(value: &ICreateTypeLib2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICreateTypeLib> for ICreateTypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICreateTypeLib> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICreateTypeLib>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICreateTypeLib> for &ICreateTypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICreateTypeLib> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICreateTypeLib>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib2_abi(
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
        szname: super::super::Foundation::PWSTR,
        tkind: TYPEKIND,
        ppctinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wmajorvernum: u16,
        wminorvernum: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szdoc: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szhelpfilename: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lcid: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulibflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pvarval: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwhelpstringcontext: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szfilename: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl IDLDESC {}
impl ::std::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IDLDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IDLDESC")
            .field("dwReserved", &self.dwReserved)
            .field("wIDLFlags", &self.wIDLFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved && self.wIDLFlags == other.wIDLFlags
    }
}
impl ::std::cmp::Eq for IDLDESC {}
unsafe impl ::windows::runtime::Abi for IDLDESC {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IDLFLAG_FIN: u32 = 1u32;
pub const IDLFLAG_FLCID: u32 = 4u32;
pub const IDLFLAG_FOUT: u32 = 2u32;
pub const IDLFLAG_FRETVAL: u32 = 8u32;
pub const IDLFLAG_NONE: u32 = 0u32;
pub const ID_DEFAULTINST: i32 = -2i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDispError(::windows::runtime::IUnknown);
impl IDispError {
    pub unsafe fn QueryErrorInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        guiderrortype: Param0,
    ) -> ::windows::runtime::Result<IDispError> {
        let mut result__: <IDispError as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            guiderrortype.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDispError>(result__)
    }
    pub unsafe fn GetNext(&self) -> ::windows::runtime::Result<IDispError> {
        let mut result__: <IDispError as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDispError>(result__)
    }
    pub unsafe fn GetHresult(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpInfo(
        &self,
        pbstrfilename: *mut super::super::Foundation::BSTR,
        pdwcontext: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbstrfilename),
            ::std::mem::transmute(pdwcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(
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
}
unsafe impl ::windows::runtime::Interface for IDispError {
    type Vtable = IDispError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2800719969,
        50976,
        4560,
        [147, 55, 0, 160, 201, 13, 202, 169],
    );
}
impl ::std::convert::From<IDispError> for ::windows::runtime::IUnknown {
    fn from(value: IDispError) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispError> for ::windows::runtime::IUnknown {
    fn from(value: &IDispError) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDispError {
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
pub struct IDispError_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guiderrortype: ::windows::runtime::GUID,
        ppde: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppde: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phr: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrsource: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrfilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwcontext: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IDispatch(::windows::runtime::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const DISPPARAMS,
        pvarresult: *mut VARIANT,
        pexcepinfo: *mut EXCEPINFO,
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
unsafe impl ::windows::runtime::Interface for IDispatch {
    type Vtable = IDispatch_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132096, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDispatch> for ::windows::runtime::IUnknown {
    fn from(value: IDispatch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispatch> for ::windows::runtime::IUnknown {
    fn from(value: &IDispatch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDispatch {
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
pub struct IDispatch_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDispatchEx(::windows::runtime::IUnknown);
impl IDispatchEx {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const DISPPARAMS,
        pvarresult: *mut VARIANT,
        pexcepinfo: *mut EXCEPINFO,
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
    pub unsafe fn GetDispID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrname: Param0,
        grfdex: u32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            bstrname.into_param().abi(),
            ::std::mem::transmute(grfdex),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn InvokeEx<
        'a,
        Param6: ::windows::runtime::IntoParam<'a, super::SystemServices::IServiceProvider>,
    >(
        &self,
        id: i32,
        lcid: u32,
        wflags: u16,
        pdp: *const DISPPARAMS,
        pvarres: *mut VARIANT,
        pei: *mut EXCEPINFO,
        pspcaller: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(id),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdp),
            ::std::mem::transmute(pvarres),
            ::std::mem::transmute(pei),
            pspcaller.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteMemberByName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrname: Param0,
        grfdex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrname.into_param().abi(),
            ::std::mem::transmute(grfdex),
        )
        .ok()
    }
    pub unsafe fn DeleteMemberByDispID(&self, id: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(id),
        )
        .ok()
    }
    pub unsafe fn GetMemberProperties(
        &self,
        id: i32,
        grfdexfetch: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(id),
            ::std::mem::transmute(grfdexfetch),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberName(
        &self,
        id: i32,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(id),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetNextDispID(&self, grfdex: u32, id: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfdex),
            ::std::mem::transmute(id),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn GetNameSpaceParent(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDispatchEx {
    type Vtable = IDispatchEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2800719968,
        50976,
        4560,
        [147, 55, 0, 160, 201, 13, 202, 169],
    );
}
impl ::std::convert::From<IDispatchEx> for ::windows::runtime::IUnknown {
    fn from(value: IDispatchEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispatchEx> for ::windows::runtime::IUnknown {
    fn from(value: &IDispatchEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDispatchEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDispatchEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDispatchEx> for IDispatch {
    fn from(value: IDispatchEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDispatchEx> for IDispatch {
    fn from(value: &IDispatchEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDispatch> for IDispatchEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDispatch>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDispatch> for &IDispatchEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDispatch>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatchEx_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        grfdex: u32,
        pid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: i32,
        lcid: u32,
        wflags: u16,
        pdp: *const DISPPARAMS,
        pvarres: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pei: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
        pspcaller: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        grfdex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: i32,
        grfdexfetch: u32,
        pgrfdex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: i32,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfdex: u32,
        id: i32,
        pid: *mut i32,
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
pub struct IEnumVARIANT(::windows::runtime::IUnknown);
impl IEnumVARIANT {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgvar: *mut VARIANT,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgvar),
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
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumVARIANT> {
        let mut result__: <IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumVARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumVARIANT {
    type Vtable = IEnumVARIANT_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132100, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumVARIANT> for ::windows::runtime::IUnknown {
    fn from(value: IEnumVARIANT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumVARIANT> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumVARIANT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumVARIANT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumVARIANT {
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
pub struct IEnumVARIANT_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgvar: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
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
pub struct IErrorInfo(::windows::runtime::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpFile(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHelpContext(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IErrorInfo {
    type Vtable = IErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        485667104,
        21629,
        4123,
        [142, 101, 8, 0, 43, 43, 209, 25],
    );
}
impl ::std::convert::From<IErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IErrorInfo {
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
pub struct IErrorInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pguid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrsource: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwhelpcontext: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IErrorLog(::windows::runtime::IUnknown);
impl IErrorLog {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddError<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpropname: Param0,
        pexcepinfo: *const EXCEPINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszpropname.into_param().abi(),
            ::std::mem::transmute(pexcepinfo),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IErrorLog {
    type Vtable = IErrorLog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        824691264,
        17518,
        4558,
        [129, 53, 0, 170, 0, 75, 184, 81],
    );
}
impl ::std::convert::From<IErrorLog> for ::windows::runtime::IUnknown {
    fn from(value: IErrorLog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IErrorLog> for ::windows::runtime::IUnknown {
    fn from(value: &IErrorLog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IErrorLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IErrorLog {
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
pub struct IErrorLog_abi(
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
        pexcepinfo: *const ::std::mem::ManuallyDrop<EXCEPINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1u32;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8u32;
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4u32;
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INTERFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERFACEDATA")
            .field("pmethdata", &self.pmethdata)
            .field("cMembers", &self.cMembers)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INTERFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pmethdata == other.pmethdata && self.cMembers == other.cMembers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INTERFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INTERFACEDATA {
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
pub struct INVOKEKIND(pub i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::std::convert::From<i32> for INVOKEKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INVOKEKIND {
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
pub struct IObjectIdentity(::windows::runtime::IUnknown);
impl IObjectIdentity {
    pub unsafe fn IsEqualObject<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        punk: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            punk.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IObjectIdentity {
    type Vtable = IObjectIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3389306854,
        3361,
        4561,
        [140, 197, 0, 192, 79, 194, 176, 133],
    );
}
impl ::std::convert::From<IObjectIdentity> for ::windows::runtime::IUnknown {
    fn from(value: IObjectIdentity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IObjectIdentity> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectIdentity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IObjectIdentity {
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
pub struct IObjectIdentity_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punk: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPropertyBag(::windows::runtime::IUnknown);
impl IPropertyBag {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Read<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, IErrorLog>,
    >(
        &self,
        pszpropname: Param0,
        pvar: *mut VARIANT,
        perrorlog: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszpropname.into_param().abi(),
            ::std::mem::transmute(pvar),
            perrorlog.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Write<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpropname: Param0,
        pvar: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszpropname.into_param().abi(),
            ::std::mem::transmute(pvar),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyBag {
    type Vtable = IPropertyBag_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1428630016,
        17099,
        4558,
        [129, 53, 0, 170, 0, 75, 184, 81],
    );
}
impl ::std::convert::From<IPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyBag) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyBag) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyBag {
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
pub struct IPropertyBag_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpropname: super::super::Foundation::PWSTR,
        pvar: *mut ::std::mem::ManuallyDrop<VARIANT>,
        perrorlog: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpropname: super::super::Foundation::PWSTR,
        pvar: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProvideRuntimeContext(::windows::runtime::IUnknown);
impl IProvideRuntimeContext {
    pub unsafe fn GetCurrentSourceContext(
        &self,
        pdwcontext: *mut usize,
        pfexecutingglobalcode: *mut i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwcontext),
            ::std::mem::transmute(pfexecutingglobalcode),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProvideRuntimeContext {
    type Vtable = IProvideRuntimeContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        283263306,
        60505,
        18898,
        [188, 81, 90, 221, 44, 54, 254, 188],
    );
}
impl ::std::convert::From<IProvideRuntimeContext> for ::windows::runtime::IUnknown {
    fn from(value: IProvideRuntimeContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideRuntimeContext> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideRuntimeContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProvideRuntimeContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProvideRuntimeContext
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
pub struct IProvideRuntimeContext_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwcontext: *mut usize,
        pfexecutingglobalcode: *mut i16,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRecordInfo(::windows::runtime::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(
        &self,
        pvnew: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvnew),
        )
        .ok()
    }
    pub unsafe fn RecordClear(
        &self,
        pvexisting: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvexisting),
        )
        .ok()
    }
    pub unsafe fn RecordCopy(
        &self,
        pvexisting: *const ::std::ffi::c_void,
        pvnew: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvexisting),
            ::std::mem::transmute(pvnew),
        )
        .ok()
    }
    pub unsafe fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetTypeInfo(&self) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetField<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pvdata: *const ::std::ffi::c_void,
        szfieldname: Param1,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvdata),
            szfieldname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetFieldNoCopy<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pvdata: *const ::std::ffi::c_void,
        szfieldname: Param1,
        pvarfield: *mut VARIANT,
        ppvdatacarray: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvdata),
            szfieldname.into_param().abi(),
            ::std::mem::transmute(pvarfield),
            ::std::mem::transmute(ppvdatacarray),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn PutField<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wflags: u32,
        pvdata: *mut ::std::ffi::c_void,
        szfieldname: Param2,
        pvarfield: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pvdata),
            szfieldname.into_param().abi(),
            ::std::mem::transmute(pvarfield),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn PutFieldNoCopy<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wflags: u32,
        pvdata: *mut ::std::ffi::c_void,
        szfieldname: Param2,
        pvarfield: *const VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pvdata),
            szfieldname.into_param().abi(),
            ::std::mem::transmute(pvarfield),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFieldNames(
        &self,
        pcnames: *mut u32,
        rgbstrnames: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcnames),
            ::std::mem::transmute(rgbstrnames),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMatchingType<'a, Param0: ::windows::runtime::IntoParam<'a, IRecordInfo>>(
        &self,
        precordinfo: Param0,
    ) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            precordinfo.into_param().abi(),
        ))
    }
    pub unsafe fn RecordCreate(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn RecordCreateCopy(
        &self,
        pvsource: *const ::std::ffi::c_void,
        ppvdest: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvsource),
            ::std::mem::transmute(ppvdest),
        )
        .ok()
    }
    pub unsafe fn RecordDestroy(
        &self,
        pvrecord: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvrecord),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRecordInfo {
    type Vtable = IRecordInfo_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(47, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRecordInfo> for ::windows::runtime::IUnknown {
    fn from(value: IRecordInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRecordInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IRecordInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRecordInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRecordInfo {
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
pub struct IRecordInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvnew: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvexisting: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvexisting: *const ::std::ffi::c_void,
        pvnew: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pguid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbsize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptypeinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvdata: *const ::std::ffi::c_void,
        szfieldname: super::super::Foundation::PWSTR,
        pvarfield: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvdata: *const ::std::ffi::c_void,
        szfieldname: super::super::Foundation::PWSTR,
        pvarfield: *mut ::std::mem::ManuallyDrop<VARIANT>,
        ppvdatacarray: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wflags: u32,
        pvdata: *mut ::std::ffi::c_void,
        szfieldname: super::super::Foundation::PWSTR,
        pvarfield: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wflags: u32,
        pvdata: *mut ::std::ffi::c_void,
        szfieldname: super::super::Foundation::PWSTR,
        pvarfield: *const ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcnames: *mut u32,
        rgbstrnames: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        precordinfo: ::windows::runtime::RawPtr,
    ) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvsource: *const ::std::ffi::c_void,
        ppvdest: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvrecord: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISupportErrorInfo(::windows::runtime::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(
        &self,
        riid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISupportErrorInfo {
    type Vtable = ISupportErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3742055776,
        21647,
        4123,
        [142, 101, 8, 0, 43, 43, 209, 25],
    );
}
impl ::std::convert::From<ISupportErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISupportErrorInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISupportErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISupportErrorInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISupportErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISupportErrorInfo {
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
pub struct ISupportErrorInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeChangeEvents(::windows::runtime::IUnknown);
impl ITypeChangeEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestTypeChange<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ITypeInfo>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        changekind: CHANGEKIND,
        ptinfobefore: Param1,
        pstrname: Param2,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(changekind),
            ptinfobefore.into_param().abi(),
            pstrname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AfterTypeChange<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ITypeInfo>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        changekind: CHANGEKIND,
        ptinfoafter: Param1,
        pstrname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(changekind),
            ptinfoafter.into_param().abi(),
            pstrname.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITypeChangeEvents {
    type Vtable = ITypeChangeEvents_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132112, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeChangeEvents> for ::windows::runtime::IUnknown {
    fn from(value: ITypeChangeEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeChangeEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeChangeEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeChangeEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeChangeEvents {
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
pub struct ITypeChangeEvents_abi(
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
        changekind: CHANGEKIND,
        ptinfobefore: ::windows::runtime::RawPtr,
        pstrname: super::super::Foundation::PWSTR,
        pfcancel: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        changekind: CHANGEKIND,
        ptinfoafter: ::windows::runtime::RawPtr,
        pstrname: super::super::Foundation::PWSTR,
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
pub struct ITypeComp(::windows::runtime::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Bind<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
        lhashval: u32,
        wflags: u16,
        pptinfo: *mut ::std::option::Option<ITypeInfo>,
        pdesckind: *mut DESCKIND,
        pbindptr: *mut BINDPTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pptinfo),
            ::std::mem::transmute(pdesckind),
            ::std::mem::transmute(pbindptr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BindType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szname: Param0,
        lhashval: u32,
        pptinfo: *mut ::std::option::Option<ITypeInfo>,
        pptcomp: *mut ::std::option::Option<ITypeComp>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            szname.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(pptinfo),
            ::std::mem::transmute(pptcomp),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITypeComp {
    type Vtable = ITypeComp_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132099, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeComp> for ::windows::runtime::IUnknown {
    fn from(value: ITypeComp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeComp> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeComp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeComp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeComp {
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
pub struct ITypeComp_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
        lhashval: u32,
        wflags: u16,
        pptinfo: *mut ::windows::runtime::RawPtr,
        pdesckind: *mut DESCKIND,
        pbindptr: *mut ::std::mem::ManuallyDrop<BINDPTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szname: super::super::Foundation::PWSTR,
        lhashval: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
        pptcomp: *mut ::windows::runtime::RawPtr,
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
pub struct ITypeFactory(::windows::runtime::IUnknown);
impl ITypeFactory {
    pub unsafe fn CreateFromTypeInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ITypeInfo>>(
        &self,
        ptypeinfo: Param0,
        riid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ptypeinfo.into_param().abi(),
            ::std::mem::transmute(riid),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITypeFactory {
    type Vtable = ITypeFactory_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(46, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeFactory> for ::windows::runtime::IUnknown {
    fn from(value: ITypeFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeFactory {
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
pub struct ITypeFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptypeinfo: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppv: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeInfo(::windows::runtime::IUnknown);
impl ITypeInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::runtime::Result<*mut TYPEATTR> {
        let mut result__: <*mut TYPEATTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::runtime::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ITypeComp>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::runtime::Result<*mut FUNCDESC> {
        let mut result__: <*mut FUNCDESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::runtime::Result<*mut VARDESC> {
        let mut result__: <*mut VARDESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<*mut VARDESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(
        &self,
        memid: i32,
        rgbstrnames: *mut super::super::Foundation::BSTR,
        cmaxnames: u32,
        pcnames: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(rgbstrnames),
            ::std::mem::transmute(cmaxnames),
            ::std::mem::transmute(pcnames),
        )
        .ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        pmemid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(pmemid),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Invoke(
        &self,
        pvinstance: *const ::std::ffi::c_void,
        memid: i32,
        wflags: u16,
        pdispparams: *mut DISPPARAMS,
        pvarresult: *mut VARIANT,
        pexcepinfo: *mut EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvinstance),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(
        &self,
        memid: i32,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrdocstring: *mut super::super::Foundation::BSTR,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pbstrdocstring),
            ::std::mem::transmute(pdwhelpcontext),
            ::std::mem::transmute(pbstrhelpfile),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
        pbstrdllname: *mut super::super::Foundation::BSTR,
        pbstrname: *mut super::super::Foundation::BSTR,
        pwordinal: *mut u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
            ::std::mem::transmute(pbstrdllname),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pwordinal),
        )
        .ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hreftype),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
        ppv: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
            ::std::mem::transmute(ppv),
        )
        .ok()
    }
    pub unsafe fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        punkouter: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            punkouter.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(
        &self,
        memid: i32,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(
        &self,
        pptlib: *mut ::std::option::Option<ITypeLib>,
        pindex: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pptlib),
            ::std::mem::transmute(pindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptypeattr),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfuncdesc),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvardesc),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for ITypeInfo {
    type Vtable = ITypeInfo_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132097, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: ITypeInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeInfo {
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
pub struct ITypeInfo_abi(
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
        pptypeattr: *mut *mut TYPEATTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptcomp: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ppfuncdesc: *mut *mut FUNCDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ppvardesc: *mut *mut VARDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        rgbstrnames: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        cmaxnames: u32,
        pcnames: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        preftype: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pimpltypeflags: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        pmemid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvinstance: *const ::std::ffi::c_void,
        memid: i32,
        wflags: u16,
        pdispparams: *mut DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdocstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
        pbstrdllname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pwordinal: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hreftype: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
        ppv: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punkouter: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvobj: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        pbstrmops: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptlib: *mut ::windows::runtime::RawPtr,
        pindex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptypeattr: *const TYPEATTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeInfo2(::windows::runtime::IUnknown);
impl ITypeInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::runtime::Result<*mut TYPEATTR> {
        let mut result__: <*mut TYPEATTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::runtime::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ITypeComp>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::runtime::Result<*mut FUNCDESC> {
        let mut result__: <*mut FUNCDESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::runtime::Result<*mut VARDESC> {
        let mut result__: <*mut VARDESC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<*mut VARDESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(
        &self,
        memid: i32,
        rgbstrnames: *mut super::super::Foundation::BSTR,
        cmaxnames: u32,
        pcnames: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(rgbstrnames),
            ::std::mem::transmute(cmaxnames),
            ::std::mem::transmute(pcnames),
        )
        .ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        pmemid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(pmemid),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Invoke(
        &self,
        pvinstance: *const ::std::ffi::c_void,
        memid: i32,
        wflags: u16,
        pdispparams: *mut DISPPARAMS,
        pvarresult: *mut VARIANT,
        pexcepinfo: *mut EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvinstance),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(
        &self,
        memid: i32,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrdocstring: *mut super::super::Foundation::BSTR,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pbstrdocstring),
            ::std::mem::transmute(pdwhelpcontext),
            ::std::mem::transmute(pbstrhelpfile),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
        pbstrdllname: *mut super::super::Foundation::BSTR,
        pbstrname: *mut super::super::Foundation::BSTR,
        pwordinal: *mut u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
            ::std::mem::transmute(pbstrdllname),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pwordinal),
        )
        .ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hreftype),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
        ppv: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
            ::std::mem::transmute(ppv),
        )
        .ok()
    }
    pub unsafe fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        punkouter: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            punkouter.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(
        &self,
        memid: i32,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(
        &self,
        pptlib: *mut ::std::option::Option<ITypeLib>,
        pindex: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pptlib),
            ::std::mem::transmute(pindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptypeattr),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfuncdesc),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvardesc),
        ))
    }
    pub unsafe fn GetTypeKind(&self) -> ::windows::runtime::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetFuncIndexOfMemId(
        &self,
        memid: i32,
        invkind: INVOKEKIND,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(invkind),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetCustData(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetFuncCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetParamCustData(
        &self,
        indexfunc: u32,
        indexparam: u32,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(indexfunc),
            ::std::mem::transmute(indexparam),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetVarCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetImplTypeCustData(
        &self,
        index: u32,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(
        &self,
        memid: i32,
        lcid: u32,
        pbstrhelpstring: *mut super::super::Foundation::BSTR,
        pdwhelpstringcontext: *mut u32,
        pbstrhelpstringdll: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(memid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(pbstrhelpstring),
            ::std::mem::transmute(pdwhelpstringcontext),
            ::std::mem::transmute(pbstrhelpstringdll),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllParamCustData(
        &self,
        indexfunc: u32,
        indexparam: u32,
    ) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(indexfunc),
            ::std::mem::transmute(indexparam),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllImplTypeCustData(
        &self,
        index: u32,
    ) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITypeInfo2 {
    type Vtable = ITypeInfo2_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132114, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeInfo2> for ::windows::runtime::IUnknown {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITypeInfo2> for ITypeInfo {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeInfo2> for ITypeInfo {
    fn from(value: &ITypeInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITypeInfo> for ITypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITypeInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITypeInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITypeInfo> for &ITypeInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITypeInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITypeInfo>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_abi(
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
        pptypeattr: *mut *mut TYPEATTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptcomp: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ppfuncdesc: *mut *mut FUNCDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ppvardesc: *mut *mut VARDESC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        rgbstrnames: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        cmaxnames: u32,
        pcnames: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        preftype: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pimpltypeflags: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        pmemid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvinstance: *const ::std::ffi::c_void,
        memid: i32,
        wflags: u16,
        pdispparams: *mut DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdocstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
        pbstrdllname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pwordinal: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hreftype: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
        ppv: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        punkouter: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        ppvobj: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        pbstrmops: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptlib: *mut ::windows::runtime::RawPtr,
        pindex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptypeattr: *const TYPEATTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptypekind: *mut TYPEKIND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptypeflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        invkind: INVOKEKIND,
        pfuncindex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        pvarindex: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        indexfunc: u32,
        indexparam: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memid: i32,
        lcid: u32,
        pbstrhelpstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpstringcontext: *mut u32,
        pbstrhelpstringdll: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        indexfunc: u32,
        indexparam: u32,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeLib(::windows::runtime::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::runtime::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::runtime::Result<*mut TLIBATTR> {
        let mut result__: <*mut TLIBATTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::runtime::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(
        &self,
        index: i32,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrdocstring: *mut super::super::Foundation::BSTR,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pbstrdocstring),
            ::std::mem::transmute(pdwhelpcontext),
            ::std::mem::transmute(pbstrhelpfile),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        sznamebuf: Param0,
        lhashval: u32,
        pfname: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            sznamebuf.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(pfname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        sznamebuf: Param0,
        lhashval: u32,
        pptinfo: *mut ::std::option::Option<ITypeInfo>,
        rgmemid: *mut i32,
        pcfound: *mut u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            sznamebuf.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(pptinfo),
            ::std::mem::transmute(rgmemid),
            ::std::mem::transmute(pcfound),
        )
        .ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptlibattr),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for ITypeLib {
    type Vtable = ITypeLib_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132098, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeLib> for ::windows::runtime::IUnknown {
    fn from(value: ITypeLib) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeLib> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeLib) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeLib {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeLib {
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
pub struct ITypeLib_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ptkind: *mut TYPEKIND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptlibattr: *mut *mut TLIBATTR,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptcomp: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdocstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sznamebuf: super::super::Foundation::PWSTR,
        lhashval: u32,
        pfname: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sznamebuf: super::super::Foundation::PWSTR,
        lhashval: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
        rgmemid: *mut i32,
        pcfound: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptlibattr: *const TLIBATTR),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeLib2(::windows::runtime::IUnknown);
impl ITypeLib2 {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::runtime::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::runtime::Result<*mut TLIBATTR> {
        let mut result__: <*mut TLIBATTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::runtime::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(
        &self,
        index: i32,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrdocstring: *mut super::super::Foundation::BSTR,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pbstrdocstring),
            ::std::mem::transmute(pdwhelpcontext),
            ::std::mem::transmute(pbstrhelpfile),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        sznamebuf: Param0,
        lhashval: u32,
        pfname: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            sznamebuf.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(pfname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        sznamebuf: Param0,
        lhashval: u32,
        pptinfo: *mut ::std::option::Option<ITypeInfo>,
        rgmemid: *mut i32,
        pcfound: *mut u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            sznamebuf.into_param().abi(),
            ::std::mem::transmute(lhashval),
            ::std::mem::transmute(pptinfo),
            ::std::mem::transmute(rgmemid),
            ::std::mem::transmute(pcfound),
        )
        .ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptlibattr),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetCustData(
        &self,
        guid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(guid),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    pub unsafe fn GetLibStatistics(
        &self,
        pcuniquenames: *mut u32,
        pcchuniquenames: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcuniquenames),
            ::std::mem::transmute(pcchuniquenames),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(
        &self,
        index: i32,
        lcid: u32,
        pbstrhelpstring: *mut super::super::Foundation::BSTR,
        pdwhelpstringcontext: *mut u32,
        pbstrhelpstringdll: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(pbstrhelpstring),
            ::std::mem::transmute(pdwhelpstringcontext),
            ::std::mem::transmute(pbstrhelpstringdll),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::runtime::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<CUSTDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITypeLib2 {
    type Vtable = ITypeLib2_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(132113, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeLib2> for ::windows::runtime::IUnknown {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeLib2> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeLib2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITypeLib2> for ITypeLib {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeLib2> for ITypeLib {
    fn from(value: &ITypeLib2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITypeLib> for ITypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITypeLib> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITypeLib>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITypeLib> for &ITypeLib2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITypeLib> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITypeLib>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        ptkind: *mut TYPEKIND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptlibattr: *mut *mut TLIBATTR,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptcomp: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrdocstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpcontext: *mut u32,
        pbstrhelpfile: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sznamebuf: super::super::Foundation::PWSTR,
        lhashval: u32,
        pfname: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sznamebuf: super::super::Foundation::PWSTR,
        lhashval: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
        rgmemid: *mut i32,
        pcfound: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptlibattr: *const TLIBATTR),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        guid: *const ::windows::runtime::GUID,
        pvarval: *mut ::std::mem::ManuallyDrop<VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcuniquenames: *mut u32,
        pcchuniquenames: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        lcid: u32,
        pbstrhelpstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pdwhelpstringcontext: *mut u32,
        pbstrhelpstringdll: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcustdata: *mut CUSTDATA,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITypeLibRegistration(::windows::runtime::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetLcid(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin32Path(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin64Path(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(
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
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpDir(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITypeLibRegistration {
    type Vtable = ITypeLibRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1990453045,
        735,
        18962,
        [152, 235, 4, 58, 211, 96, 10, 243],
    );
}
impl ::std::convert::From<ITypeLibRegistration> for ::windows::runtime::IUnknown {
    fn from(value: ITypeLibRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeLibRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeLibRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeLibRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeLibRegistration {
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
pub struct ITypeLibRegistration_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pguid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pversion: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plcid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwin32path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwin64path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdisplayname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pflags: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phelpdir: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct ITypeLibRegistrationReader(::windows::runtime::IUnknown);
impl ITypeLibRegistrationReader {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTypeLibRegistrations(
        &self,
    ) -> ::windows::runtime::Result<super::Com::IEnumUnknown> {
        let mut result__: <super::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Com::IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3983182378,
        45408,
        20087,
        [143, 115, 170, 116, 53, 205, 92, 39],
    );
}
impl ::std::convert::From<ITypeLibRegistrationReader> for ::windows::runtime::IUnknown {
    fn from(value: ITypeLibRegistrationReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeLibRegistrationReader> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeLibRegistrationReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ITypeLibRegistrationReader
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ITypeLibRegistrationReader
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
pub struct ITypeLibRegistrationReader_abi(
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
        ppenumunknown: *mut ::windows::runtime::RawPtr,
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
pub struct ITypeMarshal(::windows::runtime::IUnknown);
impl ITypeMarshal {
    pub unsafe fn Size(
        &self,
        pvtype: *const ::std::ffi::c_void,
        dwdestcontext: u32,
        pvdestcontext: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvtype),
            ::std::mem::transmute(dwdestcontext),
            ::std::mem::transmute(pvdestcontext),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn Marshal(
        &self,
        pvtype: *const ::std::ffi::c_void,
        dwdestcontext: u32,
        pvdestcontext: *const ::std::ffi::c_void,
        cbbufferlength: u32,
        pbuffer: *mut u8,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvtype),
            ::std::mem::transmute(dwdestcontext),
            ::std::mem::transmute(pvdestcontext),
            ::std::mem::transmute(cbbufferlength),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pcbwritten),
        )
        .ok()
    }
    pub unsafe fn Unmarshal(
        &self,
        pvtype: *mut ::std::ffi::c_void,
        dwflags: u32,
        cbbufferlength: u32,
        pbuffer: *const u8,
        pcbread: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvtype),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(cbbufferlength),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pcbread),
        )
        .ok()
    }
    pub unsafe fn Free(&self, pvtype: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvtype),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITypeMarshal {
    type Vtable = ITypeMarshal_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(45, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ITypeMarshal> for ::windows::runtime::IUnknown {
    fn from(value: ITypeMarshal) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITypeMarshal> for ::windows::runtime::IUnknown {
    fn from(value: &ITypeMarshal) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITypeMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITypeMarshal {
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
pub struct ITypeMarshal_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvtype: *const ::std::ffi::c_void,
        dwdestcontext: u32,
        pvdestcontext: *const ::std::ffi::c_void,
        psize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvtype: *const ::std::ffi::c_void,
        dwdestcontext: u32,
        pvdestcontext: *const ::std::ffi::c_void,
        cbbufferlength: u32,
        pbuffer: *mut u8,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvtype: *mut ::std::ffi::c_void,
        dwflags: u32,
        cbbufferlength: u32,
        pbuffer: *const u8,
        pcbread: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvtype: *const ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IVariantChangeType(::windows::runtime::IUnknown);
impl IVariantChangeType {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn ChangeType(
        &self,
        pvardst: *mut VARIANT,
        pvarsrc: *const VARIANT,
        lcid: u32,
        vtnew: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvardst),
            ::std::mem::transmute(pvarsrc),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(vtnew),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVariantChangeType {
    type Vtable = IVariantChangeType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2800719970,
        50976,
        4560,
        [147, 55, 0, 160, 201, 13, 202, 169],
    );
}
impl ::std::convert::From<IVariantChangeType> for ::windows::runtime::IUnknown {
    fn from(value: IVariantChangeType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVariantChangeType> for ::windows::runtime::IUnknown {
    fn from(value: &IVariantChangeType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVariantChangeType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVariantChangeType {
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
pub struct IVariantChangeType_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvardst: *mut ::std::mem::ManuallyDrop<VARIANT>,
        pvarsrc: *const ::std::mem::ManuallyDrop<VARIANT>,
        lcid: u32,
        vtnew: u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))] usize,
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LHashValOfNameSys<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    syskind: SYSKIND,
    lcid: u32,
    szname: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LHashValOfNameSys(
                syskind: SYSKIND,
                lcid: u32,
                szname: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(LHashValOfNameSys(
            ::std::mem::transmute(syskind),
            ::std::mem::transmute(lcid),
            szname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LHashValOfNameSysA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    syskind: SYSKIND,
    lcid: u32,
    szname: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LHashValOfNameSysA(
                syskind: SYSKIND,
                lcid: u32,
                szname: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(LHashValOfNameSysA(
            ::std::mem::transmute(syskind),
            ::std::mem::transmute(lcid),
            szname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct LIBFLAGS(pub i32);
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = LIBFLAGS(1i32);
pub const LIBFLAG_FCONTROL: LIBFLAGS = LIBFLAGS(2i32);
pub const LIBFLAG_FHIDDEN: LIBFLAGS = LIBFLAGS(4i32);
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = LIBFLAGS(8i32);
impl ::std::convert::From<i32> for LIBFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LIBFLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
pub const LOCALE_USE_NLS: u32 = 268435456u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = unsafe extern "system" fn(
    pexcepinfo: *mut ::std::mem::ManuallyDrop<EXCEPINFO>,
) -> ::windows::runtime::HRESULT;
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const SAFEARRAY);
        }
        ::std::mem::transmute(LPSAFEARRAY_UserFree(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const SAFEARRAY);
        }
        ::std::mem::transmute(LPSAFEARRAY_UserFree64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserMarshal(
    param0: *const u32,
    param1: *mut u8,
    param2: *const *const SAFEARRAY,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal(
                param0: *const u32,
                param1: *mut u8,
                param2: *const *const SAFEARRAY,
            ) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserMarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserMarshal64(
    param0: *const u32,
    param1: *mut u8,
    param2: *const *const SAFEARRAY,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal64(
                param0: *const u32,
                param1: *mut u8,
                param2: *const *const SAFEARRAY,
            ) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserMarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserSize(
    param0: *const u32,
    param1: u32,
    param2: *const *const SAFEARRAY,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize(
                param0: *const u32,
                param1: u32,
                param2: *const *const SAFEARRAY,
            ) -> u32;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserSize(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserSize64(
    param0: *const u32,
    param1: u32,
    param2: *const *const SAFEARRAY,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize64(
                param0: *const u32,
                param1: u32,
                param2: *const *const SAFEARRAY,
            ) -> u32;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserSize64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserUnmarshal(
    param0: *const u32,
    param1: *const u8,
    param2: *mut *mut SAFEARRAY,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal(
                param0: *const u32,
                param1: *const u8,
                param2: *mut *mut SAFEARRAY,
            ) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserUnmarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(
    param0: *const u32,
    param1: *const u8,
    param2: *mut *mut SAFEARRAY,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal64(
                param0: *const u32,
                param1: *const u8,
                param2: *mut *mut SAFEARRAY,
            ) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserUnmarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LoadRegTypeLib(
    rguid: *const ::windows::runtime::GUID,
    wvermajor: u16,
    wverminor: u16,
    lcid: u32,
) -> ::windows::runtime::Result<ITypeLib> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadRegTypeLib(
                rguid: *const ::windows::runtime::GUID,
                wvermajor: u16,
                wverminor: u16,
                lcid: u32,
                pptlib: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ITypeLib as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        LoadRegTypeLib(
            ::std::mem::transmute(rguid),
            ::std::mem::transmute(wvermajor),
            ::std::mem::transmute(wverminor),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<ITypeLib>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadTypeLib<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szfile: Param0,
) -> ::windows::runtime::Result<ITypeLib> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadTypeLib(
                szfile: super::super::Foundation::PWSTR,
                pptlib: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ITypeLib as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        LoadTypeLib(szfile.into_param().abi(), &mut result__).from_abi::<ITypeLib>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoadTypeLibEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szfile: Param0,
    regkind: REGKIND,
) -> ::windows::runtime::Result<ITypeLib> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadTypeLibEx(
                szfile: super::super::Foundation::PWSTR,
                regkind: REGKIND,
                pptlib: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ITypeLib as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        LoadTypeLibEx(
            szfile.into_param().abi(),
            ::std::mem::transmute(regkind),
            &mut result__,
        )
        .from_abi::<ITypeLib>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MEMBERID_NIL: i32 = -1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct METHODDATA {
    pub szName: super::super::Foundation::PWSTR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: i32,
    pub iMeth: u32,
    pub cc: CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl METHODDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for METHODDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for METHODDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("METHODDATA")
            .field("szName", &self.szName)
            .field("ppdata", &self.ppdata)
            .field("dispid", &self.dispid)
            .field("iMeth", &self.iMeth)
            .field("cc", &self.cc)
            .field("cArgs", &self.cArgs)
            .field("wFlags", &self.wFlags)
            .field("vtReturn", &self.vtReturn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for METHODDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName
            && self.ppdata == other.ppdata
            && self.dispid == other.dispid
            && self.iMeth == other.iMeth
            && self.cc == other.cc
            && self.cArgs == other.cArgs
            && self.wFlags == other.wFlags
            && self.vtReturn == other.vtReturn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for METHODDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for METHODDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: u32,
    pub dwOutFlags: u32,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
impl NUMPARSE {}
impl ::std::default::Default for NUMPARSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NUMPARSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NUMPARSE")
            .field("cDig", &self.cDig)
            .field("dwInFlags", &self.dwInFlags)
            .field("dwOutFlags", &self.dwOutFlags)
            .field("cchUsed", &self.cchUsed)
            .field("nBaseShift", &self.nBaseShift)
            .field("nPwr10", &self.nPwr10)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NUMPARSE {
    fn eq(&self, other: &Self) -> bool {
        self.cDig == other.cDig
            && self.dwInFlags == other.dwInFlags
            && self.dwOutFlags == other.dwOutFlags
            && self.cchUsed == other.cchUsed
            && self.nBaseShift == other.nBaseShift
            && self.nPwr10 == other.nPwr10
    }
}
impl ::std::cmp::Eq for NUMPARSE {}
unsafe impl ::windows::runtime::Abi for NUMPARSE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NUMPRS_CURRENCY: u32 = 1024u32;
pub const NUMPRS_DECIMAL: u32 = 256u32;
pub const NUMPRS_EXPONENT: u32 = 2048u32;
pub const NUMPRS_HEX_OCT: u32 = 64u32;
pub const NUMPRS_INEXACT: u32 = 131072u32;
pub const NUMPRS_LEADING_MINUS: u32 = 16u32;
pub const NUMPRS_LEADING_PLUS: u32 = 4u32;
pub const NUMPRS_LEADING_WHITE: u32 = 1u32;
pub const NUMPRS_NEG: u32 = 65536u32;
pub const NUMPRS_PARENS: u32 = 128u32;
pub const NUMPRS_STD: u32 = 8191u32;
pub const NUMPRS_THOUSANDS: u32 = 512u32;
pub const NUMPRS_TRAILING_MINUS: u32 = 32u32;
pub const NUMPRS_TRAILING_PLUS: u32 = 8u32;
pub const NUMPRS_TRAILING_WHITE: u32 = 2u32;
pub const NUMPRS_USE_ALL: u32 = 4096u32;
pub unsafe fn OaBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OaBuildVersion() -> u32;
        }
        ::std::mem::transmute(OaBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn OaEnablePerUserTLibRegistration() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OaEnablePerUserTLibRegistration();
        }
        ::std::mem::transmute(OaEnablePerUserTLibRegistration())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn OleLoadPictureFile<'a, Param0: ::windows::runtime::IntoParam<'a, VARIANT>>(
    varfilename: Param0,
) -> ::windows::runtime::Result<IDispatch> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPictureFile(
                varfilename: ::std::mem::ManuallyDrop<VARIANT>,
                lplpdisppicture: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleLoadPictureFile(varfilename.into_param().abi(), &mut result__)
            .from_abi::<IDispatch>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn OleLoadPictureFileEx<'a, Param0: ::windows::runtime::IntoParam<'a, VARIANT>>(
    varfilename: Param0,
    xsizedesired: u32,
    ysizedesired: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<IDispatch> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPictureFileEx(
                varfilename: ::std::mem::ManuallyDrop<VARIANT>,
                xsizedesired: u32,
                ysizedesired: u32,
                dwflags: u32,
                lplpdisppicture: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleLoadPictureFileEx(
            varfilename.into_param().abi(),
            ::std::mem::transmute(xsizedesired),
            ::std::mem::transmute(ysizedesired),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<IDispatch>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OleSavePictureFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IDispatch>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
>(
    lpdisppicture: Param0,
    bstrfilename: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSavePictureFile(
                lpdisppicture: ::windows::runtime::RawPtr,
                bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        OleSavePictureFile(
            lpdisppicture.into_param().abi(),
            bstrfilename.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PARAMDATA {
    pub szName: super::super::Foundation::PWSTR,
    pub vt: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl PARAMDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PARAMDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PARAMDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PARAMDATA")
            .field("szName", &self.szName)
            .field("vt", &self.vt)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PARAMDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.vt == other.vt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PARAMDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PARAMDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for PARAMDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for PARAMDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PARAMDESC")
            .field("pparamdescex", &self.pparamdescex)
            .field("wParamFlags", &self.wParamFlags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for PARAMDESC {
    fn eq(&self, other: &Self) -> bool {
        self.pparamdescex == other.pparamdescex && self.wParamFlags == other.wParamFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for PARAMDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl PARAMDESCEX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for PARAMDESCEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for PARAMDESCEX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for PARAMDESCEX {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64u32;
pub const PARAMFLAG_FHASDEFAULT: u32 = 32u32;
pub const PARAMFLAG_FIN: u32 = 1u32;
pub const PARAMFLAG_FLCID: u32 = 4u32;
pub const PARAMFLAG_FOPT: u32 = 16u32;
pub const PARAMFLAG_FOUT: u32 = 2u32;
pub const PARAMFLAG_FRETVAL: u32 = 8u32;
pub const PARAMFLAG_NONE: u32 = 0u32;
pub unsafe fn QueryPathOfRegTypeLib(
    guid: *const ::windows::runtime::GUID,
    wmaj: u16,
    wmin: u16,
    lcid: u32,
) -> ::windows::runtime::Result<*mut u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPathOfRegTypeLib(
                guid: *const ::windows::runtime::GUID,
                wmaj: u16,
                wmin: u16,
                lcid: u32,
                lpbstrpathname: *mut *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        QueryPathOfRegTypeLib(
            ::std::mem::transmute(guid),
            ::std::mem::transmute(wmaj),
            ::std::mem::transmute(wmin),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct REGKIND(pub i32);
pub const REGKIND_DEFAULT: REGKIND = REGKIND(0i32);
pub const REGKIND_REGISTER: REGKIND = REGKIND(1i32);
pub const REGKIND_NONE: REGKIND = REGKIND(2i32);
impl ::std::convert::From<i32> for REGKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGKIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn RegisterActiveObject<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
>(
    punk: Param0,
    rclsid: *const ::windows::runtime::GUID,
    dwflags: u32,
    pdwregister: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterActiveObject(
                punk: ::windows::runtime::RawPtr,
                rclsid: *const ::windows::runtime::GUID,
                dwflags: u32,
                pdwregister: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        RegisterActiveObject(
            punk.into_param().abi(),
            ::std::mem::transmute(rclsid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdwregister),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RegisterTypeLib<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ITypeLib>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    ptlib: Param0,
    szfullpath: Param1,
    szhelpdir: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTypeLib(
                ptlib: ::windows::runtime::RawPtr,
                szfullpath: super::super::Foundation::PWSTR,
                szhelpdir: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        RegisterTypeLib(
            ptlib.into_param().abi(),
            szfullpath.into_param().abi(),
            szhelpdir.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RegisterTypeLibForUser<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ITypeLib>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    ptlib: Param0,
    szfullpath: Param1,
    szhelpdir: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTypeLibForUser(
                ptlib: ::windows::runtime::RawPtr,
                szfullpath: super::super::Foundation::PWSTR,
                szhelpdir: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        RegisterTypeLibForUser(
            ptlib.into_param().abi(),
            szfullpath.into_param().abi(),
            szhelpdir.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RevokeActiveObject(
    dwregister: u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RevokeActiveObject(
                dwregister: u32,
                pvreserved: *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        RevokeActiveObject(
            ::std::mem::transmute(dwregister),
            ::std::mem::transmute(pvreserved),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::std::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl SAFEARRAY {}
impl ::std::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SAFEARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFEARRAY")
            .field("cDims", &self.cDims)
            .field("fFeatures", &self.fFeatures)
            .field("cbElements", &self.cbElements)
            .field("cLocks", &self.cLocks)
            .field("pvData", &self.pvData)
            .field("rgsabound", &self.rgsabound)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cDims == other.cDims
            && self.fFeatures == other.fFeatures
            && self.cbElements == other.cbElements
            && self.cLocks == other.cLocks
            && self.pvData == other.pvData
            && self.rgsabound == other.rgsabound
    }
}
impl ::std::cmp::Eq for SAFEARRAY {}
unsafe impl ::windows::runtime::Abi for SAFEARRAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl SAFEARRAYBOUND {}
impl ::std::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFEARRAYBOUND")
            .field("cElements", &self.cElements)
            .field("lLbound", &self.lLbound)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        self.cElements == other.cElements && self.lLbound == other.lLbound
    }
}
impl ::std::cmp::Eq for SAFEARRAYBOUND {}
unsafe impl ::windows::runtime::Abi for SAFEARRAYBOUND {
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
pub struct SF_TYPE(pub i32);
pub const SF_ERROR: SF_TYPE = SF_TYPE(10i32);
pub const SF_I1: SF_TYPE = SF_TYPE(16i32);
pub const SF_I2: SF_TYPE = SF_TYPE(2i32);
pub const SF_I4: SF_TYPE = SF_TYPE(3i32);
pub const SF_I8: SF_TYPE = SF_TYPE(20i32);
pub const SF_BSTR: SF_TYPE = SF_TYPE(8i32);
pub const SF_UNKNOWN: SF_TYPE = SF_TYPE(13i32);
pub const SF_DISPATCH: SF_TYPE = SF_TYPE(9i32);
pub const SF_VARIANT: SF_TYPE = SF_TYPE(12i32);
pub const SF_RECORD: SF_TYPE = SF_TYPE(36i32);
pub const SF_HAVEIID: SF_TYPE = SF_TYPE(32781i32);
impl ::std::convert::From<i32> for SF_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SF_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SID_GetCaller: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1192741952,
    48313,
    4560,
    [147, 54, 0, 160, 201, 13, 202, 169],
);
pub const SID_ProvideRuntimeContext: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1956971532,
        56588,
        18672,
        [172, 133, 25, 76, 50, 89, 24, 10],
    );
pub const SID_VariantConversion: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    521147521,
    48333,
    4560,
    [147, 54, 0, 160, 201, 13, 202, 169],
);
pub const STDOLE2_LCID: u32 = 0u32;
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
pub const STDOLE_LCID: u32 = 0u32;
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
pub const STDOLE_MINORVERNUM: u32 = 0u32;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::Com::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            );
        }
        ::std::mem::transmute(STGMEDIUM_UserFree(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::Com::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree64(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            );
        }
        ::std::mem::transmute(STGMEDIUM_UserFree64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserMarshal(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::Com::STGMEDIUM,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserMarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserMarshal64(
    param0: *const u32,
    param1: *mut u8,
    param2: *const super::Com::STGMEDIUM,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal64(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserMarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserSize(
    param0: *const u32,
    param1: u32,
    param2: *const super::Com::STGMEDIUM,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> u32;
        }
        ::std::mem::transmute(STGMEDIUM_UserSize(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserSize64(
    param0: *const u32,
    param1: u32,
    param2: *const super::Com::STGMEDIUM,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize64(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> u32;
        }
        ::std::mem::transmute(STGMEDIUM_UserSize64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserUnmarshal(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::Com::STGMEDIUM,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserUnmarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_Storage_StructuredStorage",
    feature = "Win32_System_Com"
))]
pub unsafe fn STGMEDIUM_UserUnmarshal64(
    param0: *const u32,
    param1: *const u8,
    param2: *mut super::Com::STGMEDIUM,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal64(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserUnmarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct SYSKIND(pub i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::std::convert::From<i32> for SYSKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSKIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn SafeArrayAccessData(
    psa: *const SAFEARRAY,
    ppvdata: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayAccessData(
                psa: *const SAFEARRAY,
                ppvdata: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayAccessData(::std::mem::transmute(psa), ::std::mem::transmute(ppvdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayAddRef(
    psa: *const SAFEARRAY,
    ppdatatorelease: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayAddRef(
                psa: *const SAFEARRAY,
                ppdatatorelease: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayAddRef(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(ppdatatorelease),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayAllocData(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayAllocData(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayAllocData(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayAllocDescriptor(cdims: u32) -> ::windows::runtime::Result<*mut SAFEARRAY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayAllocDescriptor(
                cdims: u32,
                ppsaout: *mut *mut SAFEARRAY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayAllocDescriptor(::std::mem::transmute(cdims), &mut result__)
            .from_abi::<*mut SAFEARRAY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayAllocDescriptorEx(
    vt: u16,
    cdims: u32,
) -> ::windows::runtime::Result<*mut SAFEARRAY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayAllocDescriptorEx(
                vt: u16,
                cdims: u32,
                ppsaout: *mut *mut SAFEARRAY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayAllocDescriptorEx(
            ::std::mem::transmute(vt),
            ::std::mem::transmute(cdims),
            &mut result__,
        )
        .from_abi::<*mut SAFEARRAY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCopy(psa: *const SAFEARRAY) -> ::windows::runtime::Result<*mut SAFEARRAY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCopy(
                psa: *const SAFEARRAY,
                ppsaout: *mut *mut SAFEARRAY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayCopy(::std::mem::transmute(psa), &mut result__)
            .from_abi::<*mut SAFEARRAY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCopyData(
    psasource: *const SAFEARRAY,
    psatarget: *const SAFEARRAY,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCopyData(
                psasource: *const SAFEARRAY,
                psatarget: *const SAFEARRAY,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayCopyData(
            ::std::mem::transmute(psasource),
            ::std::mem::transmute(psatarget),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCreate(
    vt: u16,
    cdims: u32,
    rgsabound: *const SAFEARRAYBOUND,
) -> *mut SAFEARRAY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCreate(
                vt: u16,
                cdims: u32,
                rgsabound: *const SAFEARRAYBOUND,
            ) -> *mut SAFEARRAY;
        }
        ::std::mem::transmute(SafeArrayCreate(
            ::std::mem::transmute(vt),
            ::std::mem::transmute(cdims),
            ::std::mem::transmute(rgsabound),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCreateEx(
    vt: u16,
    cdims: u32,
    rgsabound: *const SAFEARRAYBOUND,
    pvextra: *const ::std::ffi::c_void,
) -> *mut SAFEARRAY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCreateEx(
                vt: u16,
                cdims: u32,
                rgsabound: *const SAFEARRAYBOUND,
                pvextra: *const ::std::ffi::c_void,
            ) -> *mut SAFEARRAY;
        }
        ::std::mem::transmute(SafeArrayCreateEx(
            ::std::mem::transmute(vt),
            ::std::mem::transmute(cdims),
            ::std::mem::transmute(rgsabound),
            ::std::mem::transmute(pvextra),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut SAFEARRAY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut SAFEARRAY;
        }
        ::std::mem::transmute(SafeArrayCreateVector(
            ::std::mem::transmute(vt),
            ::std::mem::transmute(llbound),
            ::std::mem::transmute(celements),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayCreateVectorEx(
    vt: u16,
    llbound: i32,
    celements: u32,
    pvextra: *const ::std::ffi::c_void,
) -> *mut SAFEARRAY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayCreateVectorEx(
                vt: u16,
                llbound: i32,
                celements: u32,
                pvextra: *const ::std::ffi::c_void,
            ) -> *mut SAFEARRAY;
        }
        ::std::mem::transmute(SafeArrayCreateVectorEx(
            ::std::mem::transmute(vt),
            ::std::mem::transmute(llbound),
            ::std::mem::transmute(celements),
            ::std::mem::transmute(pvextra),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayDestroy(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayDestroy(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayDestroy(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayDestroyData(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayDestroyData(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayDestroyData(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayDestroyDescriptor(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayDestroyDescriptor(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayDestroyDescriptor(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetDim(psa: *const SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetDim(psa: *const SAFEARRAY) -> u32;
        }
        ::std::mem::transmute(SafeArrayGetDim(::std::mem::transmute(psa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetElement(
    psa: *const SAFEARRAY,
    rgindices: *const i32,
    pv: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetElement(
                psa: *const SAFEARRAY,
                rgindices: *const i32,
                pv: *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayGetElement(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(rgindices),
            ::std::mem::transmute(pv),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetElemsize(psa: *const SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetElemsize(psa: *const SAFEARRAY) -> u32;
        }
        ::std::mem::transmute(SafeArrayGetElemsize(::std::mem::transmute(psa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetIID(
    psa: *const SAFEARRAY,
) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetIID(
                psa: *const SAFEARRAY,
                pguid: *mut ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        SafeArrayGetIID(::std::mem::transmute(psa), &mut result__)
            .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetLBound(
    psa: *const SAFEARRAY,
    ndim: u32,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetLBound(
                psa: *const SAFEARRAY,
                ndim: u32,
                pllbound: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayGetLBound(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(ndim),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetRecordInfo(
    psa: *const SAFEARRAY,
) -> ::windows::runtime::Result<IRecordInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetRecordInfo(
                psa: *const SAFEARRAY,
                prinfo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRecordInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayGetRecordInfo(::std::mem::transmute(psa), &mut result__)
            .from_abi::<IRecordInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetUBound(
    psa: *const SAFEARRAY,
    ndim: u32,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetUBound(
                psa: *const SAFEARRAY,
                ndim: u32,
                plubound: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayGetUBound(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(ndim),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayGetVartype(psa: *const SAFEARRAY) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayGetVartype(
                psa: *const SAFEARRAY,
                pvt: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        SafeArrayGetVartype(::std::mem::transmute(psa), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayLock(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayLock(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayLock(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayPtrOfIndex(
    psa: *const SAFEARRAY,
    rgindices: *const i32,
    ppvdata: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayPtrOfIndex(
                psa: *const SAFEARRAY,
                rgindices: *const i32,
                ppvdata: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayPtrOfIndex(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(rgindices),
            ::std::mem::transmute(ppvdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayPutElement(
    psa: *const SAFEARRAY,
    rgindices: *const i32,
    pv: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayPutElement(
                psa: *const SAFEARRAY,
                rgindices: *const i32,
                pv: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayPutElement(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(rgindices),
            ::std::mem::transmute(pv),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayRedim(
    psa: *mut SAFEARRAY,
    psaboundnew: *const SAFEARRAYBOUND,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayRedim(
                psa: *mut SAFEARRAY,
                psaboundnew: *const SAFEARRAYBOUND,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArrayRedim(
            ::std::mem::transmute(psa),
            ::std::mem::transmute(psaboundnew),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayReleaseData(pdata: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayReleaseData(pdata: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(SafeArrayReleaseData(::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayReleaseDescriptor(psa: *const SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayReleaseDescriptor(psa: *const SAFEARRAY);
        }
        ::std::mem::transmute(SafeArrayReleaseDescriptor(::std::mem::transmute(psa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArraySetIID(
    psa: *const SAFEARRAY,
    guid: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArraySetIID(
                psa: *const SAFEARRAY,
                guid: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArraySetIID(::std::mem::transmute(psa), ::std::mem::transmute(guid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArraySetRecordInfo<'a, Param1: ::windows::runtime::IntoParam<'a, IRecordInfo>>(
    psa: *const SAFEARRAY,
    prinfo: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArraySetRecordInfo(
                psa: *const SAFEARRAY,
                prinfo: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        SafeArraySetRecordInfo(::std::mem::transmute(psa), prinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayUnaccessData(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayUnaccessData(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayUnaccessData(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SafeArrayUnlock(psa: *const SAFEARRAY) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeArrayUnlock(psa: *const SAFEARRAY) -> ::windows::runtime::HRESULT;
        }
        SafeArrayUnlock(::std::mem::transmute(psa)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetErrorInfo<'a, Param1: ::windows::runtime::IntoParam<'a, IErrorInfo>>(
    dwreserved: u32,
    perrinfo: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetErrorInfo(
                dwreserved: u32,
                perrinfo: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        SetErrorInfo(
            ::std::mem::transmute(dwreserved),
            perrinfo.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SystemTimeToVariantTime(
    lpsystemtime: *const super::super::Foundation::SYSTEMTIME,
    pvtime: *mut f64,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SystemTimeToVariantTime(
                lpsystemtime: *const super::super::Foundation::SYSTEMTIME,
                pvtime: *mut f64,
            ) -> i32;
        }
        ::std::mem::transmute(SystemTimeToVariantTime(
            ::std::mem::transmute(lpsystemtime),
            ::std::mem::transmute(pvtime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TLIBATTR {
    pub guid: ::windows::runtime::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl TLIBATTR {}
impl ::std::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TLIBATTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TLIBATTR")
            .field("guid", &self.guid)
            .field("lcid", &self.lcid)
            .field("syskind", &self.syskind)
            .field("wMajorVerNum", &self.wMajorVerNum)
            .field("wMinorVerNum", &self.wMinorVerNum)
            .field("wLibFlags", &self.wLibFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.lcid == other.lcid
            && self.syskind == other.syskind
            && self.wMajorVerNum == other.wMajorVerNum
            && self.wMinorVerNum == other.wMinorVerNum
            && self.wLibFlags == other.wLibFlags
    }
}
impl ::std::cmp::Eq for TLIBATTR {}
unsafe impl ::windows::runtime::Abi for TLIBATTR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TYPEATTR {
    pub guid: ::windows::runtime::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(feature = "Win32_Foundation")]
impl TYPEATTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TYPEATTR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TYPEATTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TYPEATTR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
impl TYPEDESC {}
impl ::std::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TYPEDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TYPEDESC {}
unsafe impl ::windows::runtime::Abi for TYPEDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut ARRAYDESC,
    pub hreftype: u32,
}
impl TYPEDESC_0 {}
impl ::std::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TYPEDESC_0 {}
unsafe impl ::windows::runtime::Abi for TYPEDESC_0 {
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
pub struct TYPEFLAGS(pub i32);
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = TYPEFLAGS(1i32);
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = TYPEFLAGS(2i32);
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = TYPEFLAGS(4i32);
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = TYPEFLAGS(8i32);
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = TYPEFLAGS(16i32);
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = TYPEFLAGS(32i32);
pub const TYPEFLAG_FDUAL: TYPEFLAGS = TYPEFLAGS(64i32);
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = TYPEFLAGS(128i32);
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = TYPEFLAGS(256i32);
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = TYPEFLAGS(512i32);
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = TYPEFLAGS(1024i32);
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = TYPEFLAGS(2048i32);
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = TYPEFLAGS(4096i32);
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = TYPEFLAGS(8192i32);
pub const TYPEFLAG_FPROXY: TYPEFLAGS = TYPEFLAGS(16384i32);
impl ::std::convert::From<i32> for TYPEFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYPEFLAGS {
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
pub struct TYPEKIND(pub i32);
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::std::convert::From<i32> for TYPEKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYPEKIND {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct UDATE {
    pub st: super::super::Foundation::SYSTEMTIME,
    pub wDayOfYear: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl UDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UDATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for UDATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UDATE")
            .field("st", &self.st)
            .field("wDayOfYear", &self.wDayOfYear)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UDATE {
    fn eq(&self, other: &Self) -> bool {
        self.st == other.st && self.wDayOfYear == other.wDayOfYear
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UDATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UDATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn UnRegisterTypeLib(
    libid: *const ::windows::runtime::GUID,
    wvermajor: u16,
    wverminor: u16,
    lcid: u32,
    syskind: SYSKIND,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnRegisterTypeLib(
                libid: *const ::windows::runtime::GUID,
                wvermajor: u16,
                wverminor: u16,
                lcid: u32,
                syskind: SYSKIND,
            ) -> ::windows::runtime::HRESULT;
        }
        UnRegisterTypeLib(
            ::std::mem::transmute(libid),
            ::std::mem::transmute(wvermajor),
            ::std::mem::transmute(wverminor),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(syskind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn UnRegisterTypeLibForUser(
    libid: *const ::windows::runtime::GUID,
    wmajorvernum: u16,
    wminorvernum: u16,
    lcid: u32,
    syskind: SYSKIND,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnRegisterTypeLibForUser(
                libid: *const ::windows::runtime::GUID,
                wmajorvernum: u16,
                wminorvernum: u16,
                lcid: u32,
                syskind: SYSKIND,
            ) -> ::windows::runtime::HRESULT;
        }
        UnRegisterTypeLibForUser(
            ::std::mem::transmute(libid),
            ::std::mem::transmute(wmajorvernum),
            ::std::mem::transmute(wminorvernum),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(syskind),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const VARCMP_EQ: u32 = 1u32;
pub const VARCMP_GT: u32 = 2u32;
pub const VARCMP_LT: u32 = 0u32;
pub const VARCMP_NULL: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut ::std::mem::ManuallyDrop<VARIANT>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARDESC_0 {
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
pub struct VARENUM(pub i32);
pub const VT_EMPTY: VARENUM = VARENUM(0i32);
pub const VT_NULL: VARENUM = VARENUM(1i32);
pub const VT_I2: VARENUM = VARENUM(2i32);
pub const VT_I4: VARENUM = VARENUM(3i32);
pub const VT_R4: VARENUM = VARENUM(4i32);
pub const VT_R8: VARENUM = VARENUM(5i32);
pub const VT_CY: VARENUM = VARENUM(6i32);
pub const VT_DATE: VARENUM = VARENUM(7i32);
pub const VT_BSTR: VARENUM = VARENUM(8i32);
pub const VT_DISPATCH: VARENUM = VARENUM(9i32);
pub const VT_ERROR: VARENUM = VARENUM(10i32);
pub const VT_BOOL: VARENUM = VARENUM(11i32);
pub const VT_VARIANT: VARENUM = VARENUM(12i32);
pub const VT_UNKNOWN: VARENUM = VARENUM(13i32);
pub const VT_DECIMAL: VARENUM = VARENUM(14i32);
pub const VT_I1: VARENUM = VARENUM(16i32);
pub const VT_UI1: VARENUM = VARENUM(17i32);
pub const VT_UI2: VARENUM = VARENUM(18i32);
pub const VT_UI4: VARENUM = VARENUM(19i32);
pub const VT_I8: VARENUM = VARENUM(20i32);
pub const VT_UI8: VARENUM = VARENUM(21i32);
pub const VT_INT: VARENUM = VARENUM(22i32);
pub const VT_UINT: VARENUM = VARENUM(23i32);
pub const VT_VOID: VARENUM = VARENUM(24i32);
pub const VT_HRESULT: VARENUM = VARENUM(25i32);
pub const VT_PTR: VARENUM = VARENUM(26i32);
pub const VT_SAFEARRAY: VARENUM = VARENUM(27i32);
pub const VT_CARRAY: VARENUM = VARENUM(28i32);
pub const VT_USERDEFINED: VARENUM = VARENUM(29i32);
pub const VT_LPSTR: VARENUM = VARENUM(30i32);
pub const VT_LPWSTR: VARENUM = VARENUM(31i32);
pub const VT_RECORD: VARENUM = VARENUM(36i32);
pub const VT_INT_PTR: VARENUM = VARENUM(37i32);
pub const VT_UINT_PTR: VARENUM = VARENUM(38i32);
pub const VT_FILETIME: VARENUM = VARENUM(64i32);
pub const VT_BLOB: VARENUM = VARENUM(65i32);
pub const VT_STREAM: VARENUM = VARENUM(66i32);
pub const VT_STORAGE: VARENUM = VARENUM(67i32);
pub const VT_STREAMED_OBJECT: VARENUM = VARENUM(68i32);
pub const VT_STORED_OBJECT: VARENUM = VARENUM(69i32);
pub const VT_BLOB_OBJECT: VARENUM = VARENUM(70i32);
pub const VT_CF: VARENUM = VARENUM(71i32);
pub const VT_CLSID: VARENUM = VARENUM(72i32);
pub const VT_VERSIONED_STREAM: VARENUM = VARENUM(73i32);
pub const VT_BSTR_BLOB: VARENUM = VARENUM(4095i32);
pub const VT_VECTOR: VARENUM = VARENUM(4096i32);
pub const VT_ARRAY: VARENUM = VARENUM(8192i32);
pub const VT_BYREF: VARENUM = VARENUM(16384i32);
pub const VT_RESERVED: VARENUM = VARENUM(32768i32);
pub const VT_ILLEGAL: VARENUM = VARENUM(65535i32);
pub const VT_ILLEGALMASKED: VARENUM = VARENUM(4095i32);
pub const VT_TYPEMASK: VARENUM = VARENUM(4095i32);
impl ::std::convert::From<i32> for VARENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VARENUM {
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
pub struct VARFLAGS(pub i32);
pub const VARFLAG_FREADONLY: VARFLAGS = VARFLAGS(1i32);
pub const VARFLAG_FSOURCE: VARFLAGS = VARFLAGS(2i32);
pub const VARFLAG_FBINDABLE: VARFLAGS = VARFLAGS(4i32);
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = VARFLAGS(8i32);
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = VARFLAGS(16i32);
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = VARFLAGS(32i32);
pub const VARFLAG_FHIDDEN: VARFLAGS = VARFLAGS(64i32);
pub const VARFLAG_FRESTRICTED: VARFLAGS = VARFLAGS(128i32);
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = VARFLAGS(256i32);
pub const VARFLAG_FUIDEFAULT: VARFLAGS = VARFLAGS(512i32);
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = VARFLAGS(1024i32);
pub const VARFLAG_FREPLACEABLE: VARFLAGS = VARFLAGS(2048i32);
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = VARFLAGS(4096i32);
impl ::std::convert::From<i32> for VARFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VARFLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARIANT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union VARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::SystemServices::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: super::SystemServices::CY,
    pub date: f64,
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub punkVal: ::windows::runtime::RawPtr,
    pub pdispVal: ::windows::runtime::RawPtr,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::SystemServices::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut ::std::mem::ManuallyDrop<VARIANT>,
    pub byref: *mut ::std::ffi::c_void,
    pub cVal: super::SystemServices::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::SystemServices::DECIMAL,
    pub pcVal: super::super::Foundation::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::std::ffi::c_void,
    pub pRecInfo: ::std::option::Option<IRecordInfo>,
}
impl VARIANT_0_0_0_0 {}
impl ::std::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("pvRecord", &self.pvRecord)
            .field("pRecInfo", &self.pRecInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
impl ::std::cmp::Eq for VARIANT_0_0_0_0 {}
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const VARIANT_ALPHABOOL: u32 = 2u32;
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64u32;
pub const VARIANT_CALENDAR_HIJRI: u32 = 8u32;
pub const VARIANT_CALENDAR_THAI: u32 = 32u32;
pub const VARIANT_LOCALBOOL: u32 = 16u32;
pub const VARIANT_NOUSEROVERRIDE: u32 = 4u32;
pub const VARIANT_NOVALUEPROP: u32 = 1u32;
pub const VARIANT_USE_NLS: u32 = 128u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<VARIANT>,
            );
        }
        ::std::mem::transmute(VARIANT_UserFree(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree64(
                param0: *const u32,
                param1: *const ::std::mem::ManuallyDrop<VARIANT>,
            );
        }
        ::std::mem::transmute(VARIANT_UserFree64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserMarshal(
    param0: *const u32,
    param1: *mut u8,
    param2: *const VARIANT,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserMarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserMarshal64(
    param0: *const u32,
    param1: *mut u8,
    param2: *const VARIANT,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal64(
                param0: *const u32,
                param1: *mut u8,
                param2: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserMarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> u32;
        }
        ::std::mem::transmute(VARIANT_UserSize(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize64(
                param0: *const u32,
                param1: u32,
                param2: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> u32;
        }
        ::std::mem::transmute(VARIANT_UserSize64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserUnmarshal(
    param0: *const u32,
    param1: *const u8,
    param2: *mut VARIANT,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserUnmarshal(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VARIANT_UserUnmarshal64(
    param0: *const u32,
    param1: *const u8,
    param2: *mut VARIANT,
) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal64(
                param0: *const u32,
                param1: *const u8,
                param2: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserUnmarshal64(
            ::std::mem::transmute(param0),
            ::std::mem::transmute(param1),
            ::std::mem::transmute(param2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub struct VARKIND(pub i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::std::convert::From<i32> for VARKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VARKIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VTDATEGRE_MAX: u32 = 2958465u32;
pub const VTDATEGRE_MIN: i32 = -657434i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarAbs(pvarin: *const VARIANT) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarAbs(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarAbs(::std::mem::transmute(pvarin), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarAdd(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarAdd(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarAdd(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarAnd(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarAnd(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarAnd(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarBoolFromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromCy(
                cyin: super::SystemServices::CY,
                pboolout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromCy(cyin.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromDate(datein: f64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromDate(datein: f64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromDate(::std::mem::transmute(datein), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarBoolFromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pboolout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pboolout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarBoolFromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromI1(
                cin: super::SystemServices::CHAR,
                pboolout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromI1(cin.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromI2(sin: i16) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromI2(sin: i16, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromI2(::std::mem::transmute(sin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromI4(lin: i32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromI4(lin: i32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromI4(::std::mem::transmute(lin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromI8(i64in: i64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromI8(i64in: i64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromR4(fltin: f32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromR4(fltin: f32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromR8(dblin: f64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromR8(dblin: f64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBoolFromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pboolout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromUI1(bin: u8) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromUI1(bin: u8, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromUI2(uiin: u16) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromUI2(uiin: u16, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromUI4(ulin: u32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromUI4(ulin: u32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarBoolFromUI8(i64in: u64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBoolFromUI8(i64in: u64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBoolFromUI8(::std::mem::transmute(i64in), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrCat<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
>(
    bstrleft: Param0,
    bstrright: Param1,
) -> ::windows::runtime::Result<*mut u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrCat(
                bstrleft: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                bstrright: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                pbstrresult: *mut *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarBstrCat(
            bstrleft.into_param().abi(),
            bstrright.into_param().abi(),
            &mut result__,
        )
        .from_abi::<*mut u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrCmp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
>(
    bstrleft: Param0,
    bstrright: Param1,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrCmp(
                bstrleft: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                bstrright: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                lcid: u32,
                dwflags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        VarBstrCmp(
            bstrleft.into_param().abi(),
            bstrright.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromBool(
    boolin: i16,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromBool(
                boolin: i16,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromBool(
            ::std::mem::transmute(boolin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarBstrFromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromCy(
                cyin: super::SystemServices::CY,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromCy(
            cyin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromDate(
    datein: f64,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromDate(
                datein: f64,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromDate(
            ::std::mem::transmute(datein),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarBstrFromDec(
    pdecin: *const super::SystemServices::DECIMAL,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromDec(
            ::std::mem::transmute(pdecin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarBstrFromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromI1(
                cin: super::SystemServices::CHAR,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromI1(
            cin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromI2(
    ival: i16,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromI2(
                ival: i16,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromI2(
            ::std::mem::transmute(ival),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromI4(
    lin: i32,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromI4(
                lin: i32,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromI4(
            ::std::mem::transmute(lin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromI8(
    i64in: i64,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromI8(
                i64in: i64,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromI8(
            ::std::mem::transmute(i64in),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromR4(
    fltin: f32,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromR4(
                fltin: f32,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromR4(
            ::std::mem::transmute(fltin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromR8(
    dblin: f64,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromR8(
                dblin: f64,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromR8(
            ::std::mem::transmute(dblin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromUI1(
    bval: u8,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromUI1(
                bval: u8,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromUI1(
            ::std::mem::transmute(bval),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromUI2(
    uiin: u16,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromUI2(
                uiin: u16,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromUI2(
            ::std::mem::transmute(uiin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromUI4(
    ulin: u32,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromUI4(
                ulin: u32,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromUI4(
            ::std::mem::transmute(ulin),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarBstrFromUI8(
    ui64in: u64,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarBstrFromUI8(
                ui64in: u64,
                lcid: u32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarBstrFromUI8(
            ::std::mem::transmute(ui64in),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarCat(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCat(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarCat(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarCmp(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCmp(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                lcid: u32,
                dwflags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        VarCmp(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyAbs<'a, Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>>(
    cyin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyAbs(
                cyin: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyAbs(cyin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyAdd<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
    Param1: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    cyright: Param1,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyAdd(
                cyleft: super::SystemServices::CY,
                cyright: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyAdd(
            cyleft.into_param().abi(),
            cyright.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyCmp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
    Param1: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    cyright: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyCmp(
                cyleft: super::SystemServices::CY,
                cyright: super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        VarCyCmp(cyleft.into_param().abi(), cyright.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyCmpR8<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    dblright: f64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyCmpR8(
                cyleft: super::SystemServices::CY,
                dblright: f64,
            ) -> ::windows::runtime::HRESULT;
        }
        VarCyCmpR8(cyleft.into_param().abi(), ::std::mem::transmute(dblright)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFix<'a, Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>>(
    cyin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFix(
                cyin: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFix(cyin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromBool(boolin: i16) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromBool(
                boolin: i16,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromBool(::std::mem::transmute(boolin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromDate(datein: f64) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromDate(
                datein: f64,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromDate(::std::mem::transmute(datein), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromDec(::std::mem::transmute(pdecin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromI1(
                cin: super::SystemServices::CHAR,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromI1(cin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromI2(sin: i16) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromI2(
                sin: i16,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromI2(::std::mem::transmute(sin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromI4(lin: i32) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromI4(
                lin: i32,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromI4(::std::mem::transmute(lin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromI8(i64in: i64) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromI8(
                i64in: i64,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromI8(::std::mem::transmute(i64in), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromR4(fltin: f32) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromR4(
                fltin: f32,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromR4(::std::mem::transmute(fltin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromR8(dblin: f64) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromR8(
                dblin: f64,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromR8(::std::mem::transmute(dblin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarCyFromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromUI1(bin: u8) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromUI1(
                bin: u8,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromUI1(::std::mem::transmute(bin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromUI2(uiin: u16) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromUI2(
                uiin: u16,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromUI2(::std::mem::transmute(uiin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromUI4(ulin: u32) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromUI4(
                ulin: u32,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromUI4(::std::mem::transmute(ulin), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyFromUI8(ui64in: u64) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyFromUI8(
                ui64in: u64,
                pcyout: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyFromUI8(::std::mem::transmute(ui64in), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyInt<'a, Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>>(
    cyin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyInt(
                cyin: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyInt(cyin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyMul<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
    Param1: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    cyright: Param1,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyMul(
                cyleft: super::SystemServices::CY,
                cyright: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyMul(
            cyleft.into_param().abi(),
            cyright.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyMulI4<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    lright: i32,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyMulI4(
                cyleft: super::SystemServices::CY,
                lright: i32,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyMulI4(
            cyleft.into_param().abi(),
            ::std::mem::transmute(lright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyMulI8<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    lright: i64,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyMulI8(
                cyleft: super::SystemServices::CY,
                lright: i64,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyMulI8(
            cyleft.into_param().abi(),
            ::std::mem::transmute(lright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyNeg<'a, Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>>(
    cyin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyNeg(
                cyin: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyNeg(cyin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCyRound<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    cdecimals: i32,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCyRound(
                cyin: super::SystemServices::CY,
                cdecimals: i32,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCyRound(
            cyin.into_param().abi(),
            ::std::mem::transmute(cdecimals),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarCySub<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
    Param1: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyleft: Param0,
    cyright: Param1,
) -> ::windows::runtime::Result<super::SystemServices::CY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarCySub(
                cyleft: super::SystemServices::CY,
                cyright: super::SystemServices::CY,
                pcyresult: *mut super::SystemServices::CY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::CY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarCySub(
            cyleft.into_param().abi(),
            cyright.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::SystemServices::CY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromBool(boolin: i16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromBool(boolin: i16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDateFromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromCy(
                cyin: super::SystemServices::CY,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromCy(cyin.into_param().abi(), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDateFromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDateFromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromI1(
                cin: super::SystemServices::CHAR,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromI1(cin.into_param().abi(), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromI2(sin: i16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromI2(sin: i16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromI2(::std::mem::transmute(sin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromI4(lin: i32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromI4(lin: i32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromI4(::std::mem::transmute(lin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromI8(i64in: i64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromI8(i64in: i64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromR4(fltin: f32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromR4(fltin: f32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromR8(dblin: f64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromR8(dblin: f64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarDateFromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromUI1(bin: u8) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUI1(bin: u8, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromUI2(uiin: u16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUI2(uiin: u16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromUI4(ulin: u32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUI4(ulin: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarDateFromUI8(ui64in: u64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUI8(ui64in: u64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarDateFromUdate(
    pudatein: *const UDATE,
    dwflags: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUdate(
                pudatein: *const UDATE,
                dwflags: u32,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUdate(
            ::std::mem::transmute(pudatein),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarDateFromUdateEx(
    pudatein: *const UDATE,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDateFromUdateEx(
                pudatein: *const UDATE,
                lcid: u32,
                dwflags: u32,
                pdateout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDateFromUdateEx(
            ::std::mem::transmute(pudatein),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecAbs(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecAbs(
                pdecin: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecAbs(::std::mem::transmute(pdecin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecAdd(
    pdecleft: *const super::SystemServices::DECIMAL,
    pdecright: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecAdd(
                pdecleft: *const super::SystemServices::DECIMAL,
                pdecright: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecAdd(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(pdecright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecCmp(
    pdecleft: *const super::SystemServices::DECIMAL,
    pdecright: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecCmp(
                pdecleft: *const super::SystemServices::DECIMAL,
                pdecright: *const super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        VarDecCmp(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(pdecright),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecCmpR8(
    pdecleft: *const super::SystemServices::DECIMAL,
    dblright: f64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecCmpR8(
                pdecleft: *const super::SystemServices::DECIMAL,
                dblright: f64,
            ) -> ::windows::runtime::HRESULT;
        }
        VarDecCmpR8(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(dblright),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecDiv(
    pdecleft: *const super::SystemServices::DECIMAL,
    pdecright: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecDiv(
                pdecleft: *const super::SystemServices::DECIMAL,
                pdecright: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecDiv(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(pdecright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFix(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFix(
                pdecin: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFix(::std::mem::transmute(pdecin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromBool(
    boolin: i16,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromBool(
                boolin: i16,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromBool(::std::mem::transmute(boolin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromCy(
                cyin: super::SystemServices::CY,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromCy(cyin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromDate(
    datein: f64,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromDate(
                datein: f64,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromDate(::std::mem::transmute(datein), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromI1(
                cin: super::SystemServices::CHAR,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromI1(cin.into_param().abi(), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromI2(
    uiin: i16,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromI2(
                uiin: i16,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromI2(::std::mem::transmute(uiin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromI4(lin: i32) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromI4(
                lin: i32,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromI4(::std::mem::transmute(lin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromI8(
    i64in: i64,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromI8(
                i64in: i64,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromI8(::std::mem::transmute(i64in), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromR4(
    fltin: f32,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromR4(
                fltin: f32,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromR4(::std::mem::transmute(fltin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromR8(
    dblin: f64,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromR8(
                dblin: f64,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromR8(::std::mem::transmute(dblin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarDecFromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromUI1(bin: u8) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromUI1(
                bin: u8,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromUI1(::std::mem::transmute(bin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromUI2(
    uiin: u16,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromUI2(
                uiin: u16,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromUI2(::std::mem::transmute(uiin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromUI4(
    ulin: u32,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromUI4(
                ulin: u32,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromUI4(::std::mem::transmute(ulin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecFromUI8(
    ui64in: u64,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecFromUI8(
                ui64in: u64,
                pdecout: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecFromUI8(::std::mem::transmute(ui64in), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecInt(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecInt(
                pdecin: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecInt(::std::mem::transmute(pdecin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecMul(
    pdecleft: *const super::SystemServices::DECIMAL,
    pdecright: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecMul(
                pdecleft: *const super::SystemServices::DECIMAL,
                pdecright: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecMul(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(pdecright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecNeg(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecNeg(
                pdecin: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecNeg(::std::mem::transmute(pdecin), &mut result__)
            .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecRound(
    pdecin: *const super::SystemServices::DECIMAL,
    cdecimals: i32,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecRound(
                pdecin: *const super::SystemServices::DECIMAL,
                cdecimals: i32,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecRound(
            ::std::mem::transmute(pdecin),
            ::std::mem::transmute(cdecimals),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarDecSub(
    pdecleft: *const super::SystemServices::DECIMAL,
    pdecright: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<super::SystemServices::DECIMAL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDecSub(
                pdecleft: *const super::SystemServices::DECIMAL,
                pdecright: *const super::SystemServices::DECIMAL,
                pdecresult: *mut super::SystemServices::DECIMAL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::SystemServices::DECIMAL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarDecSub(
            ::std::mem::transmute(pdecleft),
            ::std::mem::transmute(pdecright),
            &mut result__,
        )
        .from_abi::<super::SystemServices::DECIMAL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarDiv(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarDiv(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarDiv(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarEqv(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarEqv(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarEqv(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFix(pvarin: *const VARIANT) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFix(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarFix(::std::mem::transmute(pvarin), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormat<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pvarin: *const VARIANT,
    pstrformat: Param1,
    ifirstday: i32,
    ifirstweek: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormat(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pstrformat: super::super::Foundation::PWSTR,
                ifirstday: i32,
                ifirstweek: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarFormat(
            ::std::mem::transmute(pvarin),
            pstrformat.into_param().abi(),
            ::std::mem::transmute(ifirstday),
            ::std::mem::transmute(ifirstweek),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormatCurrency(
    pvarin: *const VARIANT,
    inumdig: i32,
    iinclead: i32,
    iuseparens: i32,
    igroup: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormatCurrency(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                inumdig: i32,
                iinclead: i32,
                iuseparens: i32,
                igroup: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarFormatCurrency(
            ::std::mem::transmute(pvarin),
            ::std::mem::transmute(inumdig),
            ::std::mem::transmute(iinclead),
            ::std::mem::transmute(iuseparens),
            ::std::mem::transmute(igroup),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormatDateTime(
    pvarin: *const VARIANT,
    inamedformat: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormatDateTime(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                inamedformat: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarFormatDateTime(
            ::std::mem::transmute(pvarin),
            ::std::mem::transmute(inamedformat),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormatFromTokens<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pvarin: *const VARIANT,
    pstrformat: Param1,
    pbtokcur: *const u8,
    dwflags: u32,
    pbstrout: *mut super::super::Foundation::BSTR,
    lcid: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormatFromTokens(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pstrformat: super::super::Foundation::PWSTR,
                pbtokcur: *const u8,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                lcid: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        VarFormatFromTokens(
            ::std::mem::transmute(pvarin),
            pstrformat.into_param().abi(),
            ::std::mem::transmute(pbtokcur),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pbstrout),
            ::std::mem::transmute(lcid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormatNumber(
    pvarin: *const VARIANT,
    inumdig: i32,
    iinclead: i32,
    iuseparens: i32,
    igroup: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormatNumber(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                inumdig: i32,
                iinclead: i32,
                iuseparens: i32,
                igroup: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarFormatNumber(
            ::std::mem::transmute(pvarin),
            ::std::mem::transmute(inumdig),
            ::std::mem::transmute(iinclead),
            ::std::mem::transmute(iuseparens),
            ::std::mem::transmute(igroup),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarFormatPercent(
    pvarin: *const VARIANT,
    inumdig: i32,
    iinclead: i32,
    iuseparens: i32,
    igroup: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarFormatPercent(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                inumdig: i32,
                iinclead: i32,
                iuseparens: i32,
                igroup: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarFormatPercent(
            ::std::mem::transmute(pvarin),
            ::std::mem::transmute(inumdig),
            ::std::mem::transmute(iinclead),
            ::std::mem::transmute(iuseparens),
            ::std::mem::transmute(igroup),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromBool(
    boolin: i16,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromBool(
                boolin: i16,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromBool(::std::mem::transmute(boolin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarI1FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromCy(
                cyin: super::SystemServices::CY,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromCy(cyin.into_param().abi(), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromDate(
    datein: f64,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromDate(
                datein: f64,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromDate(::std::mem::transmute(datein), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarI1FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromDec(::std::mem::transmute(pdecin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(pcout),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromI2(
    uiin: i16,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromI2(
                uiin: i16,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromI2(::std::mem::transmute(uiin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromI4(
    lin: i32,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromI4(
                lin: i32,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromI4(::std::mem::transmute(lin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromI8(
    i64in: i64,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromI8(
                i64in: i64,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromI8(::std::mem::transmute(i64in), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromR4(
    fltin: f32,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromR4(
                fltin: f32,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromR4(::std::mem::transmute(fltin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromR8(
    dblin: f64,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromR8(
                dblin: f64,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromR8(::std::mem::transmute(dblin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pcout),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromUI1(
    bin: u8,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromUI1(
                bin: u8,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromUI1(::std::mem::transmute(bin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromUI2(
    uiin: u16,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromUI2(
                uiin: u16,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromUI2(::std::mem::transmute(uiin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromUI4(
    ulin: u32,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromUI4(
                ulin: u32,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromUI4(::std::mem::transmute(ulin), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI1FromUI8(
    i64in: u64,
    pcout: super::super::Foundation::PSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI1FromUI8(
                i64in: u64,
                pcout: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI1FromUI8(::std::mem::transmute(i64in), ::std::mem::transmute(pcout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromBool(boolin: i16) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromBool(boolin: i16, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI2FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    psout: *mut i16,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromCy(
                cyin: super::SystemServices::CY,
                psout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        VarI2FromCy(cyin.into_param().abi(), ::std::mem::transmute(psout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromDate(datein: f64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromDate(datein: f64, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI2FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                psout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                psout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI2FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromI1(
                cin: super::SystemServices::CHAR,
                psout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromI1(cin.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromI4(lin: i32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromI4(lin: i32, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromI8(i64in: i64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromI8(i64in: i64, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromR4(fltin: f32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromR4(fltin: f32, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromR8(dblin: f64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromR8(dblin: f64, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI2FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                psout: *mut i16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromUI1(bin: u8) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromUI1(bin: u8, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromUI2(uiin: u16) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromUI2(uiin: u16, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromUI4(ulin: u32) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromUI4(ulin: u32, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI2FromUI8(ui64in: u64) -> ::windows::runtime::Result<i16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI2FromUI8(ui64in: u64, psout: *mut i16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI2FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromBool(boolin: i16) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromBool(boolin: i16, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI4FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromCy(
                cyin: super::SystemServices::CY,
                plout: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromCy(cyin.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromDate(datein: f64) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromDate(datein: f64, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI4FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                plout: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                plout: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI4FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromI1(
                cin: super::SystemServices::CHAR,
                plout: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromI1(cin.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromI2(sin: i16) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromI2(sin: i16, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromI8(i64in: i64) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromI8(i64in: i64, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromR4(fltin: f32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromR4(fltin: f32, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromR8(dblin: f64) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromR8(dblin: f64, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI4FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                plout: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromUI1(bin: u8) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromUI1(bin: u8, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromUI2(uiin: u16) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromUI2(uiin: u16, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromUI4(ulin: u32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromUI4(ulin: u32, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI4FromUI8(ui64in: u64) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI4FromUI8(ui64in: u64, plout: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI4FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromBool(boolin: i16) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromBool(boolin: i16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI8FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromCy(
                cyin: super::SystemServices::CY,
                pi64out: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromCy(cyin.into_param().abi(), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromDate(datein: f64) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromDate(datein: f64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI8FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pi64out: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pi64out: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarI8FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromI1(
                cin: super::SystemServices::CHAR,
                pi64out: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromI1(cin.into_param().abi(), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromI2(sin: i16) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromI2(sin: i16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromR4(fltin: f32) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromR4(fltin: f32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromR8(dblin: f64) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromR8(dblin: f64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarI8FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pi64out: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromUI1(bin: u8) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromUI1(bin: u8, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromUI2(uiin: u16) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromUI2(uiin: u16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromUI4(ulin: u32) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromUI4(ulin: u32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarI8FromUI8(ui64in: u64) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarI8FromUI8(ui64in: u64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarI8FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarIdiv(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarIdiv(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarIdiv(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarImp(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarImp(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarImp(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarInt(pvarin: *const VARIANT) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarInt(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarInt(::std::mem::transmute(pvarin), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarMod(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarMod(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarMod(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarMonthName(
    imonth: i32,
    fabbrev: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarMonthName(
                imonth: i32,
                fabbrev: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarMonthName(
            ::std::mem::transmute(imonth),
            ::std::mem::transmute(fabbrev),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarMul(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarMul(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarMul(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarNeg(pvarin: *const VARIANT) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarNeg(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarNeg(::std::mem::transmute(pvarin), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarNot(pvarin: *const VARIANT) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarNot(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarNot(::std::mem::transmute(pvarin), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarNumFromParseNum(
    pnumprs: *const NUMPARSE,
    rgbdig: *const u8,
    dwvtbits: u32,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarNumFromParseNum(
                pnumprs: *const NUMPARSE,
                rgbdig: *const u8,
                dwvtbits: u32,
                pvar: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarNumFromParseNum(
            ::std::mem::transmute(pnumprs),
            ::std::mem::transmute(rgbdig),
            ::std::mem::transmute(dwvtbits),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarOr(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarOr(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarOr(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarParseNumFromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
    pnumprs: *mut NUMPARSE,
    rgbdig: *mut u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarParseNumFromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pnumprs: *mut NUMPARSE,
                rgbdig: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        VarParseNumFromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pnumprs),
            ::std::mem::transmute(rgbdig),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarPow(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarPow(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarPow(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows::runtime::HRESULT;
        }
        VarR4CmpR8(
            ::std::mem::transmute(fltleft),
            ::std::mem::transmute(dblright),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromBool(boolin: i16) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromBool(boolin: i16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR4FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    pfltout: *mut f32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromCy(
                cyin: super::SystemServices::CY,
                pfltout: *mut f32,
            ) -> ::windows::runtime::HRESULT;
        }
        VarR4FromCy(cyin.into_param().abi(), ::std::mem::transmute(pfltout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromDate(datein: f64) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromDate(datein: f64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR4FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pfltout: *mut f32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pfltout: *mut f32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR4FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromI1(
                cin: super::SystemServices::CHAR,
                pfltout: *mut f32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromI1(cin.into_param().abi(), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromI2(sin: i16) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromI2(sin: i16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromI4(lin: i32) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromI4(lin: i32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromI8(i64in: i64) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromI8(i64in: i64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromR8(dblin: f64) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromR8(dblin: f64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarR4FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pfltout: *mut f32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromUI1(bin: u8) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromUI1(bin: u8, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromUI2(uiin: u16) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromUI2(uiin: u16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromUI4(ulin: u32) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromUI4(ulin: u32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR4FromUI8(ui64in: u64) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR4FromUI8(ui64in: u64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR4FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromBool(boolin: i16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromBool(boolin: i16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR8FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
    pdblout: *mut f64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromCy(
                cyin: super::SystemServices::CY,
                pdblout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        VarR8FromCy(cyin.into_param().abi(), ::std::mem::transmute(pdblout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromDate(datein: f64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromDate(datein: f64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR8FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pdblout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pdblout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarR8FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
    pdblout: *mut f64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromI1(
                cin: super::SystemServices::CHAR,
                pdblout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        VarR8FromI1(cin.into_param().abi(), ::std::mem::transmute(pdblout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromI2(sin: i16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromI2(sin: i16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromI4(lin: i32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromI4(lin: i32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromI8(i64in: i64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromI8(i64in: i64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromR4(fltin: f32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromR4(fltin: f32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarR8FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pdblout: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromUI1(bin: u8) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromUI1(bin: u8, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromUI2(uiin: u16) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromUI2(uiin: u16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromUI4(ulin: u32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromUI4(ulin: u32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8FromUI8(ui64in: u64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8FromUI8(ui64in: u64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8Pow(dblleft: f64, dblright: f64) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8Pow(
                dblleft: f64,
                dblright: f64,
                pdblresult: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8Pow(
            ::std::mem::transmute(dblleft),
            ::std::mem::transmute(dblright),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarR8Round(dblin: f64, cdecimals: i32) -> ::windows::runtime::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarR8Round(
                dblin: f64,
                cdecimals: i32,
                pdblresult: *mut f64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarR8Round(
            ::std::mem::transmute(dblin),
            ::std::mem::transmute(cdecimals),
            &mut result__,
        )
        .from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarRound(
    pvarin: *const VARIANT,
    cdecimals: i32,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarRound(
                pvarin: *const ::std::mem::ManuallyDrop<VARIANT>,
                cdecimals: i32,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarRound(
            ::std::mem::transmute(pvarin),
            ::std::mem::transmute(cdecimals),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarSub(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarSub(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarSub(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarTokenizeFormatString<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pstrformat: Param0,
    rgbtok: *mut u8,
    cbtok: i32,
    ifirstday: i32,
    ifirstweek: i32,
    lcid: u32,
    pcbactual: *const i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarTokenizeFormatString(
                pstrformat: super::super::Foundation::PWSTR,
                rgbtok: *mut u8,
                cbtok: i32,
                ifirstday: i32,
                ifirstweek: i32,
                lcid: u32,
                pcbactual: *const i32,
            ) -> ::windows::runtime::HRESULT;
        }
        VarTokenizeFormatString(
            pstrformat.into_param().abi(),
            ::std::mem::transmute(rgbtok),
            ::std::mem::transmute(cbtok),
            ::std::mem::transmute(ifirstday),
            ::std::mem::transmute(ifirstweek),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(pcbactual),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromBool(boolin: i16) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromBool(boolin: i16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI1FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromCy(
                cyin: super::SystemServices::CY,
                pbout: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromCy(cyin.into_param().abi(), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromDate(datein: f64) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromDate(datein: f64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI1FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pbout: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pbout: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI1FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromI1(
                cin: super::SystemServices::CHAR,
                pbout: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromI1(cin.into_param().abi(), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromI2(sin: i16) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromI2(sin: i16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromI4(lin: i32) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromI4(lin: i32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromI8(i64in: i64) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromI8(i64in: i64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromR4(fltin: f32) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromR4(fltin: f32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromR8(dblin: f64) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromR8(dblin: f64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarUI1FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pbout: *mut u8,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromUI2(uiin: u16) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromUI2(uiin: u16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromUI4(ulin: u32) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromUI4(ulin: u32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI1FromUI8(ui64in: u64) -> ::windows::runtime::Result<u8> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI1FromUI8(ui64in: u64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI1FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<u8>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromBool(boolin: i16) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromBool(boolin: i16, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI2FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromCy(
                cyin: super::SystemServices::CY,
                puiout: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromCy(cyin.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromDate(datein: f64) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromDate(datein: f64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI2FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                puiout: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                puiout: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI2FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromI1(
                cin: super::SystemServices::CHAR,
                puiout: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromI1(cin.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromI2(uiin: i16) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromI2(uiin: i16, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromI2(::std::mem::transmute(uiin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromI4(lin: i32) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromI4(lin: i32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromI8(i64in: i64) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromI8(i64in: i64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromR4(fltin: f32) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromR4(fltin: f32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        VarUI2FromR8(::std::mem::transmute(dblin), ::std::mem::transmute(puiout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarUI2FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                puiout: *mut u16,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromUI1(bin: u8) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromUI1(bin: u8, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromUI4(ulin: u32) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromUI4(ulin: u32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI2FromUI8(i64in: u64) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI2FromUI8(i64in: u64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI2FromUI8(::std::mem::transmute(i64in), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromBool(boolin: i16) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromBool(boolin: i16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI4FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromCy(
                cyin: super::SystemServices::CY,
                pulout: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromCy(cyin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromDate(datein: f64) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromDate(datein: f64, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI4FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pulout: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pulout: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI4FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromI1(
                cin: super::SystemServices::CHAR,
                pulout: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromI1(cin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromI2(uiin: i16) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromI2(uiin: i16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromI2(::std::mem::transmute(uiin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromI4(lin: i32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromI4(lin: i32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromI4(::std::mem::transmute(lin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromI8(i64in: i64) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromI8(i64in: i64, plout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromI8(::std::mem::transmute(i64in), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromR4(fltin: f32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromR4(fltin: f32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromR8(dblin: f64) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromR8(dblin: f64, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarUI4FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pulout: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromUI1(bin: u8) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromUI1(bin: u8, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromUI2(uiin: u16) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromUI2(uiin: u16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI4FromUI8(ui64in: u64) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI4FromUI8(ui64in: u64, plout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI4FromUI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromBool(boolin: i16) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromBool(boolin: i16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromBool(::std::mem::transmute(boolin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI8FromCy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CY>,
>(
    cyin: Param0,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromCy(
                cyin: super::SystemServices::CY,
                pi64out: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromCy(cyin.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromDate(datein: f64) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromDate(datein: f64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromDate(::std::mem::transmute(datein), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI8FromDec(
    pdecin: *const super::SystemServices::DECIMAL,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromDec(
                pdecin: *const super::SystemServices::DECIMAL,
                pi64out: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromDec(::std::mem::transmute(pdecin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromDisp<'a, Param0: ::windows::runtime::IntoParam<'a, IDispatch>>(
    pdispin: Param0,
    lcid: u32,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromDisp(
                pdispin: ::windows::runtime::RawPtr,
                lcid: u32,
                pi64out: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromDisp(
            pdispin.into_param().abi(),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn VarUI8FromI1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
>(
    cin: Param0,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromI1(
                cin: super::SystemServices::CHAR,
                pi64out: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromI1(cin.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromI2(sin: i16) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromI2(sin: i16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromI2(::std::mem::transmute(sin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromI8(ui64in: i64) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromI8(ui64in: i64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromI8(::std::mem::transmute(ui64in), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromR4(fltin: f32) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromR4(fltin: f32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromR4(::std::mem::transmute(fltin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromR8(dblin: f64) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromR8(dblin: f64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromR8(::std::mem::transmute(dblin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarUI8FromStr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    strin: Param0,
    lcid: u32,
    dwflags: u32,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromStr(
                strin: super::super::Foundation::PWSTR,
                lcid: u32,
                dwflags: u32,
                pi64out: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromStr(
            strin.into_param().abi(),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromUI1(bin: u8) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromUI1(bin: u8, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromUI1(::std::mem::transmute(bin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromUI2(uiin: u16) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromUI2(uiin: u16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromUI2(::std::mem::transmute(uiin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VarUI8FromUI4(ulin: u32) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUI8FromUI4(ulin: u32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUI8FromUI4(::std::mem::transmute(ulin), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarUdateFromDate(datein: f64, dwflags: u32) -> ::windows::runtime::Result<UDATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarUdateFromDate(
                datein: f64,
                dwflags: u32,
                pudateout: *mut UDATE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <UDATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarUdateFromDate(
            ::std::mem::transmute(datein),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<UDATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VarWeekdayName(
    iweekday: i32,
    fabbrev: i32,
    ifirstday: i32,
    dwflags: u32,
) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarWeekdayName(
                iweekday: i32,
                fabbrev: i32,
                ifirstday: i32,
                dwflags: u32,
                pbstrout: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        VarWeekdayName(
            ::std::mem::transmute(iweekday),
            ::std::mem::transmute(fabbrev),
            ::std::mem::transmute(ifirstday),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VarXor(
    pvarleft: *const VARIANT,
    pvarright: *const VARIANT,
) -> ::windows::runtime::Result<VARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VarXor(
                pvarleft: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarright: *const ::std::mem::ManuallyDrop<VARIANT>,
                pvarresult: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VarXor(
            ::std::mem::transmute(pvarleft),
            ::std::mem::transmute(pvarright),
            &mut result__,
        )
        .from_abi::<VARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantChangeType(
    pvargdest: *mut VARIANT,
    pvarsrc: *const VARIANT,
    wflags: u16,
    vt: u16,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantChangeType(
                pvargdest: *mut ::std::mem::ManuallyDrop<VARIANT>,
                pvarsrc: *const ::std::mem::ManuallyDrop<VARIANT>,
                wflags: u16,
                vt: u16,
            ) -> ::windows::runtime::HRESULT;
        }
        VariantChangeType(
            ::std::mem::transmute(pvargdest),
            ::std::mem::transmute(pvarsrc),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(vt),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantChangeTypeEx(
    pvargdest: *mut VARIANT,
    pvarsrc: *const VARIANT,
    lcid: u32,
    wflags: u16,
    vt: u16,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantChangeTypeEx(
                pvargdest: *mut ::std::mem::ManuallyDrop<VARIANT>,
                pvarsrc: *const ::std::mem::ManuallyDrop<VARIANT>,
                lcid: u32,
                wflags: u16,
                vt: u16,
            ) -> ::windows::runtime::HRESULT;
        }
        VariantChangeTypeEx(
            ::std::mem::transmute(pvargdest),
            ::std::mem::transmute(pvarsrc),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(vt),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantClear(pvarg: *mut VARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantClear(
                pvarg: *mut ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        VariantClear(::std::mem::transmute(pvarg)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantCopy(
    pvargdest: *mut VARIANT,
    pvargsrc: *const VARIANT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantCopy(
                pvargdest: *mut ::std::mem::ManuallyDrop<VARIANT>,
                pvargsrc: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        VariantCopy(
            ::std::mem::transmute(pvargdest),
            ::std::mem::transmute(pvargsrc),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantCopyInd(
    pvardest: *mut VARIANT,
    pvargsrc: *const VARIANT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantCopyInd(
                pvardest: *mut ::std::mem::ManuallyDrop<VARIANT>,
                pvargsrc: *const ::std::mem::ManuallyDrop<VARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        VariantCopyInd(
            ::std::mem::transmute(pvardest),
            ::std::mem::transmute(pvargsrc),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn VariantInit(pvarg: *mut VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantInit(pvarg: *mut ::std::mem::ManuallyDrop<VARIANT>);
        }
        ::std::mem::transmute(VariantInit(::std::mem::transmute(pvarg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn VariantTimeToDosDateTime(
    vtime: f64,
    pwdosdate: *mut u16,
    pwdostime: *mut u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantTimeToDosDateTime(
                vtime: f64,
                pwdosdate: *mut u16,
                pwdostime: *mut u16,
            ) -> i32;
        }
        ::std::mem::transmute(VariantTimeToDosDateTime(
            ::std::mem::transmute(vtime),
            ::std::mem::transmute(pwdosdate),
            ::std::mem::transmute(pwdostime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VariantTimeToSystemTime(
    vtime: f64,
    lpsystemtime: *mut super::super::Foundation::SYSTEMTIME,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VariantTimeToSystemTime(
                vtime: f64,
                lpsystemtime: *mut super::super::Foundation::SYSTEMTIME,
            ) -> i32;
        }
        ::std::mem::transmute(VariantTimeToSystemTime(
            ::std::mem::transmute(vtime),
            ::std::mem::transmute(lpsystemtime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn VectorFromBstr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
>(
    bstr: Param0,
) -> ::windows::runtime::Result<*mut SAFEARRAY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VectorFromBstr(
                bstr: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
                ppsa: *mut *mut SAFEARRAY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        VectorFromBstr(bstr.into_param().abi(), &mut result__).from_abi::<*mut SAFEARRAY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: ::std::option::Option<IRecordInfo>,
    pub pRecord: *mut u8,
}
impl _wireBRECORD {}
impl ::std::default::Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _wireBRECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireBRECORD")
            .field("fFlags", &self.fFlags)
            .field("clSize", &self.clSize)
            .field("pRecInfo", &self.pRecInfo)
            .field("pRecord", &self.pRecord)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _wireBRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags
            && self.clSize == other.clSize
            && self.pRecInfo == other.pRecInfo
            && self.pRecord == other.pRecord
    }
}
impl ::std::cmp::Eq for _wireBRECORD {}
unsafe impl ::windows::runtime::Abi for _wireBRECORD {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: _wireSAFEARRAY_UNION,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireSAFEARRAY {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireSAFEARRAY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireSAFEARRAY {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireSAFEARRAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub struct _wireSAFEARRAY_UNION {
    pub sfType: u32,
    pub u: _wireSAFEARRAY_UNION_0,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireSAFEARRAY_UNION {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireSAFEARRAY_UNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireSAFEARRAY_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireSAFEARRAY_UNION {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireSAFEARRAY_UNION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub union _wireSAFEARRAY_UNION_0 {
    pub BstrStr: _wireSAFEARR_BSTR,
    pub UnknownStr: _wireSAFEARR_UNKNOWN,
    pub DispatchStr: _wireSAFEARR_DISPATCH,
    pub VariantStr: _wireSAFEARR_VARIANT,
    pub RecordStr: _wireSAFEARR_BRECORD,
    pub HaveIidStr: _wireSAFEARR_HAVEIID,
    pub ByteStr: super::Com::BYTE_SIZEDARR,
    pub WordStr: super::Com::SHORT_SIZEDARR,
    pub LongStr: super::Com::LONG_SIZEDARR,
    pub HyperStr: super::Com::HYPER_SIZEDARR,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireSAFEARRAY_UNION_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireSAFEARRAY_UNION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireSAFEARRAY_UNION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireSAFEARRAY_UNION_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireSAFEARRAY_UNION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _wireSAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut *mut _wireBRECORD,
}
impl _wireSAFEARR_BRECORD {}
impl ::std::default::Default for _wireSAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _wireSAFEARR_BRECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_BRECORD")
            .field("Size", &self.Size)
            .field("aRecord", &self.aRecord)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _wireSAFEARR_BRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aRecord == other.aRecord
    }
}
impl ::std::cmp::Eq for _wireSAFEARR_BRECORD {}
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_BRECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireSAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut *mut super::Com::FLAGGED_WORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl _wireSAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for _wireSAFEARR_BSTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for _wireSAFEARR_BSTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_BSTR")
            .field("Size", &self.Size)
            .field("aBstr", &self.aBstr)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for _wireSAFEARR_BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aBstr == other.aBstr
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for _wireSAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_BSTR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _wireSAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut ::std::option::Option<IDispatch>,
}
impl _wireSAFEARR_DISPATCH {}
impl ::std::default::Default for _wireSAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _wireSAFEARR_DISPATCH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_DISPATCH")
            .field("Size", &self.Size)
            .field("apDispatch", &self.apDispatch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _wireSAFEARR_DISPATCH {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apDispatch == other.apDispatch
    }
}
impl ::std::cmp::Eq for _wireSAFEARR_DISPATCH {}
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_DISPATCH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _wireSAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut ::std::option::Option<::windows::runtime::IUnknown>,
    pub iid: ::windows::runtime::GUID,
}
impl _wireSAFEARR_HAVEIID {}
impl ::std::default::Default for _wireSAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _wireSAFEARR_HAVEIID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_HAVEIID")
            .field("Size", &self.Size)
            .field("apUnknown", &self.apUnknown)
            .field("iid", &self.iid)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _wireSAFEARR_HAVEIID {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown && self.iid == other.iid
    }
}
impl ::std::cmp::Eq for _wireSAFEARR_HAVEIID {}
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_HAVEIID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _wireSAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut ::std::option::Option<::windows::runtime::IUnknown>,
}
impl _wireSAFEARR_UNKNOWN {}
impl ::std::default::Default for _wireSAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _wireSAFEARR_UNKNOWN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_UNKNOWN")
            .field("Size", &self.Size)
            .field("apUnknown", &self.apUnknown)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _wireSAFEARR_UNKNOWN {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown
    }
}
impl ::std::cmp::Eq for _wireSAFEARR_UNKNOWN {}
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_UNKNOWN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub struct _wireSAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireSAFEARR_VARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireSAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::fmt::Debug for _wireSAFEARR_VARIANT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wireSAFEARR_VARIANT")
            .field("Size", &self.Size)
            .field("aVariant", &self.aVariant)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireSAFEARR_VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aVariant == other.aVariant
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireSAFEARR_VARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireSAFEARR_VARIANT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireVARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireVARIANT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub scode: i32,
    pub cyVal: super::SystemServices::CY,
    pub date: f64,
    pub bstrVal: *mut super::Com::FLAGGED_WORD_BLOB,
    pub punkVal: ::windows::runtime::RawPtr,
    pub pdispVal: ::windows::runtime::RawPtr,
    pub parray: *mut *mut _wireSAFEARRAY,
    pub brecVal: *mut ::std::mem::ManuallyDrop<_wireBRECORD>,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::SystemServices::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut super::Com::FLAGGED_WORD_BLOB,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut ::std::mem::ManuallyDrop<_wireVARIANT>,
    pub cVal: super::SystemServices::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::SystemServices::DECIMAL,
    pub pdecVal: *mut super::SystemServices::DECIMAL,
    pub pcVal: super::super::Foundation::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl _wireVARIANT_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for _wireVARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for _wireVARIANT_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for _wireVARIANT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
