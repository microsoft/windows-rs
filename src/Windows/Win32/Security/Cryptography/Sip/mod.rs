#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPAddProvider(::std::mem::transmute(psnewprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPCreateIndirectData(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(pcbindirectdata), ::std::mem::transmute(pindirectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetCaps(::std::mem::transmute(psubjinfo), ::std::mem::transmute(pcaps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetSealedDigest(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(psig), ::std::mem::transmute(dwsig), ::std::mem::transmute(pbdigest), ::std::mem::transmute(pcbdigest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetSignedDataMsg(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(pdwencodingtype), ::std::mem::transmute(dwindex), ::std::mem::transmute(pcbsigneddatamsg), ::std::mem::transmute(pbsigneddatamsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPLoad(pgsubject: *const ::windows::runtime::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPLoad(pgsubject: *const ::windows::runtime::GUID, dwflags: u32, psipdispatch: *mut ::std::mem::ManuallyDrop<SIP_DISPATCH_INFO>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPLoad(::std::mem::transmute(pgsubject), ::std::mem::transmute(dwflags), ::std::mem::transmute(psipdispatch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPPutSignedDataMsg(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(dwencodingtype), ::std::mem::transmute(pdwindex), ::std::mem::transmute(cbsigneddatamsg), ::std::mem::transmute(pbsigneddatamsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CryptSIPRemoveProvider(pgprov: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveProvider(pgprov: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRemoveProvider(::std::mem::transmute(pgprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRemoveSignedDataMsg(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(dwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(filename: Param0, hfilein: Param1, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuid(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRetrieveSubjectGuid(filename.into_param().abi(), hfilein.into_param().abi(), ::std::mem::transmute(pgsubject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuidForCatalogFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(filename: Param0, hfilein: Param1, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRetrieveSubjectGuidForCatalogFile(filename.into_param().abi(), hfilein.into_param().abi(), ::std::mem::transmute(pgsubject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
#[inline]
pub unsafe fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPVerifyIndirectData(::std::mem::transmute(psubjectinfo), ::std::mem::transmute(pindirectdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl MS_ADDINFO_BLOB {}
impl ::std::default::Default for MS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MS_ADDINFO_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_BLOB").field("cbStruct", &self.cbStruct).field("cbMemObject", &self.cbMemObject).field("pbMemObject", &self.pbMemObject).field("cbMemSignedMsg", &self.cbMemSignedMsg).field("pbMemSignedMsg", &self.pbMemSignedMsg).finish()
    }
}
impl ::std::cmp::PartialEq for MS_ADDINFO_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cbMemObject == other.cbMemObject && self.pbMemObject == other.pbMemObject && self.cbMemSignedMsg == other.cbMemSignedMsg && self.pbMemSignedMsg == other.pbMemSignedMsg
    }
}
impl ::std::cmp::Eq for MS_ADDINFO_BLOB {}
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut super::Catalog::CRYPTCATSTORE,
    pub pMember: *mut super::Catalog::CRYPTCATMEMBER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::default::Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::fmt::Debug for MS_ADDINFO_CATALOGMEMBER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_CATALOGMEMBER").field("cbStruct", &self.cbStruct).field("pStore", &self.pStore).field("pMember", &self.pMember).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::PartialEq for MS_ADDINFO_CATALOGMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pStore == other.pStore && self.pMember == other.pMember
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::Eq for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_CATALOGMEMBER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl MS_ADDINFO_FLAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MS_ADDINFO_FLAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_FLAT").field("cbStruct", &self.cbStruct).field("pIndirectData", &self.pIndirectData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MS_ADDINFO_FLAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pIndirectData == other.pIndirectData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MS_ADDINFO_FLAT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_FLAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut ::windows::runtime::GUID,
    pub pwszDLLFileName: super::super::super::Foundation::PWSTR,
    pub pwszMagicNumber: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionName: super::super::super::Foundation::PWSTR,
    pub pwszGetFuncName: super::super::super::Foundation::PWSTR,
    pub pwszPutFuncName: super::super::super::Foundation::PWSTR,
    pub pwszCreateFuncName: super::super::super::Foundation::PWSTR,
    pub pwszVerifyFuncName: super::super::super::Foundation::PWSTR,
    pub pwszRemoveFuncName: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionNameFmt2: super::super::super::Foundation::PWSTR,
    pub pwszGetCapFuncName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SIP_ADD_NEWPROVIDER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_ADD_NEWPROVIDER")
            .field("cbStruct", &self.cbStruct)
            .field("pgSubject", &self.pgSubject)
            .field("pwszDLLFileName", &self.pwszDLLFileName)
            .field("pwszMagicNumber", &self.pwszMagicNumber)
            .field("pwszIsFunctionName", &self.pwszIsFunctionName)
            .field("pwszGetFuncName", &self.pwszGetFuncName)
            .field("pwszPutFuncName", &self.pwszPutFuncName)
            .field("pwszCreateFuncName", &self.pwszCreateFuncName)
            .field("pwszVerifyFuncName", &self.pwszVerifyFuncName)
            .field("pwszRemoveFuncName", &self.pwszRemoveFuncName)
            .field("pwszIsFunctionNameFmt2", &self.pwszIsFunctionNameFmt2)
            .field("pwszGetCapFuncName", &self.pwszGetCapFuncName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_ADD_NEWPROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pgSubject == other.pgSubject
            && self.pwszDLLFileName == other.pwszDLLFileName
            && self.pwszMagicNumber == other.pwszMagicNumber
            && self.pwszIsFunctionName == other.pwszIsFunctionName
            && self.pwszGetFuncName == other.pwszGetFuncName
            && self.pwszPutFuncName == other.pwszPutFuncName
            && self.pwszCreateFuncName == other.pwszCreateFuncName
            && self.pwszVerifyFuncName == other.pwszVerifyFuncName
            && self.pwszRemoveFuncName == other.pwszRemoveFuncName
            && self.pwszIsFunctionNameFmt2 == other.pwszIsFunctionNameFmt2
            && self.pwszGetCapFuncName == other.pwszGetCapFuncName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_ADD_NEWPROVIDER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_CAP_SET_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SIP_CAP_SET_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_CAP_SET_V2").field("cbSize", &self.cbSize).field("dwVersion", &self.dwVersion).field("isMultiSign", &self.isMultiSign).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_CAP_SET_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVersion == other.dwVersion && self.isMultiSign == other.isMultiSign && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_CAP_SET_V3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl SIP_CAP_SET_V3_0 {}
impl ::std::default::Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SIP_CAP_SET_V3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SIP_CAP_SET_V3_0 {}
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V3_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::super::super::Foundation::HANDLE,
    pub pfGet: ::std::option::Option<pCryptSIPGetSignedDataMsg>,
    pub pfPut: ::std::option::Option<pCryptSIPPutSignedDataMsg>,
    pub pfCreate: ::std::option::Option<pCryptSIPCreateIndirectData>,
    pub pfVerify: ::std::option::Option<pCryptSIPVerifyIndirectData>,
    pub pfRemove: ::std::option::Option<pCryptSIPRemoveSignedDataMsg>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl SIP_DISPATCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::default::Default for SIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::fmt::Debug for SIP_DISPATCH_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_DISPATCH_INFO").field("cbSize", &self.cbSize).field("hSIP", &self.hSIP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::PartialEq for SIP_DISPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hSIP == other.hSIP && self.pfGet.map(|f| f as usize) == other.pfGet.map(|f| f as usize) && self.pfPut.map(|f| f as usize) == other.pfPut.map(|f| f as usize) && self.pfCreate.map(|f| f as usize) == other.pfCreate.map(|f| f as usize) && self.pfVerify.map(|f| f as usize) == other.pfVerify.map(|f| f as usize) && self.pfRemove.map(|f| f as usize) == other.pfRemove.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::Eq for SIP_DISPATCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::runtime::Abi for SIP_DISPATCH_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_INDIRECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_INDIRECT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SIP_INDIRECT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_INDIRECT_DATA").field("Data", &self.Data).field("DigestAlgorithm", &self.DigestAlgorithm).field("Digest", &self.Digest).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_INDIRECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data && self.DigestAlgorithm == other.DigestAlgorithm && self.Digest == other.Digest
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_INDIRECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_INDIRECT_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut ::windows::runtime::GUID,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub pwsFileName: super::super::super::Foundation::PWSTR,
    pub pwsDisplayName: super::super::super::Foundation::PWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl SIP_SUBJECTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::default::Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::PartialEq for SIP_SUBJECTINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::Eq for SIP_SUBJECTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::runtime::Abi for SIP_SUBJECTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl SIP_SUBJECTINFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::default::Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::PartialEq for SIP_SUBJECTINFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::std::cmp::Eq for SIP_SUBJECTINFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
unsafe impl ::windows::runtime::Abi for SIP_SUBJECTINFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_Sip`*"]
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPCreateIndirectData = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetCaps = unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSealedDigest = unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPPutSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPRemoveSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPVerifyIndirectData = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupported = unsafe extern "system" fn(hfile: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupportedName = unsafe extern "system" fn(pwszfilename: super::super::super::Foundation::PWSTR, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
