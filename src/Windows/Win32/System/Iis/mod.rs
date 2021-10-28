#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ADMINDATA_MAX_NAME_LEN: u32 = 256u32;
pub const APPCTR_MD_ID_BEGIN_RESERVED: u32 = 57344u32;
pub const APPCTR_MD_ID_END_RESERVED: u32 = 61439u32;
pub const APPSTATUS_NOTDEFINED: u32 = 2u32;
pub const APPSTATUS_RUNNING: u32 = 1u32;
pub const APPSTATUS_STOPPED: u32 = 0u32;
pub const ASP_MD_ID_BEGIN_RESERVED: u32 = 28672u32;
pub const ASP_MD_ID_END_RESERVED: u32 = 29951u32;
pub const ASP_MD_SERVER_BASE: u32 = 7000u32;
pub const ASP_MD_UT_APP: u32 = 101u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AsyncIFtpAuthenticationProvider(::windows::runtime::IUnknown);
impl AsyncIFtpAuthenticationProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_AuthenticateUser<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
        pszpassword: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
            pszpassword.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_AuthenticateUser(
        &self,
        ppszcanonicalusername: *mut super::super::Foundation::PWSTR,
        pfauthenticated: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppszcanonicalusername),
            ::std::mem::transmute(pfauthenticated),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpAuthenticationProvider {
    type Vtable = AsyncIFtpAuthenticationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3259956069,
        40766,
        18838,
        [143, 177, 206, 22, 105, 22, 186, 181],
    );
}
impl ::std::convert::From<AsyncIFtpAuthenticationProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpAuthenticationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpAuthenticationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpAuthenticationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIFtpAuthenticationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpAuthenticationProvider
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
pub struct AsyncIFtpAuthenticationProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        pszpassword: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppszcanonicalusername: *mut super::super::Foundation::PWSTR,
        pfauthenticated: *mut super::super::Foundation::BOOL,
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
pub struct AsyncIFtpAuthorizationProvider(::windows::runtime::IUnknown);
impl AsyncIFtpAuthorizationProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_GetUserAccessPermission<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszvirtualpath: Param2,
        pszusername: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszvirtualpath.into_param().abi(),
            pszusername.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Finish_GetUserAccessPermission(&self) -> ::windows::runtime::Result<FTP_ACCESS> {
        let mut result__: <FTP_ACCESS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<FTP_ACCESS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpAuthorizationProvider {
    type Vtable = AsyncIFtpAuthorizationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2249048889,
        2021,
        19036,
        [156, 97, 136, 32, 206, 160, 18, 188],
    );
}
impl ::std::convert::From<AsyncIFtpAuthorizationProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpAuthorizationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpAuthorizationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpAuthorizationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIFtpAuthorizationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpAuthorizationProvider
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
pub struct AsyncIFtpAuthorizationProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszvirtualpath: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pftpaccess: *mut FTP_ACCESS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AsyncIFtpHomeDirectoryProvider(::windows::runtime::IUnknown);
impl AsyncIFtpHomeDirectoryProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_GetUserHomeDirectoryData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_GetUserHomeDirectoryData(
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
unsafe impl ::windows::runtime::Interface for AsyncIFtpHomeDirectoryProvider {
    type Vtable = AsyncIFtpHomeDirectoryProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1945638456,
        25237,
        17085,
        [162, 190, 74, 101, 127, 124, 71, 156],
    );
}
impl ::std::convert::From<AsyncIFtpHomeDirectoryProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpHomeDirectoryProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpHomeDirectoryProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpHomeDirectoryProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIFtpHomeDirectoryProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpHomeDirectoryProvider
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
pub struct AsyncIFtpHomeDirectoryProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppszhomedirectorydata: *mut super::super::Foundation::PWSTR,
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
pub struct AsyncIFtpLogProvider(::windows::runtime::IUnknown);
impl AsyncIFtpLogProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_Log(
        &self,
        ploggingparameters: *const LOGGING_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ploggingparameters),
        )
        .ok()
    }
    pub unsafe fn Finish_Log(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpLogProvider {
    type Vtable = AsyncIFtpLogProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        10530374,
        9368,
        18610,
        [149, 230, 223, 103, 142, 215, 212, 159],
    );
}
impl ::std::convert::From<AsyncIFtpLogProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpLogProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpLogProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpLogProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIFtpLogProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIFtpLogProvider {
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
pub struct AsyncIFtpLogProvider_abi(
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
        ploggingparameters: *const LOGGING_PARAMETERS,
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
pub struct AsyncIFtpPostprocessProvider(::windows::runtime::IUnknown);
impl AsyncIFtpPostprocessProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePostprocess(
        &self,
        ppostprocessparameters: *const POST_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppostprocessparameters),
        )
        .ok()
    }
    pub unsafe fn Finish_HandlePostprocess(
        &self,
    ) -> ::windows::runtime::Result<FTP_PROCESS_STATUS> {
        let mut result__: <FTP_PROCESS_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpPostprocessProvider {
    type Vtable = AsyncIFtpPostprocessProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2708153666,
        38548,
        20145,
        [165, 100, 108, 46, 145, 253, 193, 51],
    );
}
impl ::std::convert::From<AsyncIFtpPostprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpPostprocessProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpPostprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpPostprocessProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIFtpPostprocessProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpPostprocessProvider
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
pub struct AsyncIFtpPostprocessProvider_abi(
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
        ppostprocessparameters: *const POST_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pftpprocessstatus: *mut FTP_PROCESS_STATUS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AsyncIFtpPreprocessProvider(::windows::runtime::IUnknown);
impl AsyncIFtpPreprocessProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePreprocess(
        &self,
        ppreprocessparameters: *const PRE_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppreprocessparameters),
        )
        .ok()
    }
    pub unsafe fn Finish_HandlePreprocess(&self) -> ::windows::runtime::Result<FTP_PROCESS_STATUS> {
        let mut result__: <FTP_PROCESS_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpPreprocessProvider {
    type Vtable = AsyncIFtpPreprocessProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1878392207,
        64910,
        18609,
        [163, 224, 191, 112, 115, 219, 77, 181],
    );
}
impl ::std::convert::From<AsyncIFtpPreprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpPreprocessProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpPreprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpPreprocessProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIFtpPreprocessProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpPreprocessProvider
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
pub struct AsyncIFtpPreprocessProvider_abi(
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
        ppreprocessparameters: *const PRE_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pftpprocessstatus: *mut FTP_PROCESS_STATUS,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AsyncIFtpRoleProvider(::windows::runtime::IUnknown);
impl AsyncIFtpRoleProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_IsUserInRole<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
        pszrole: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
            pszrole.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsUserInRole(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIFtpRoleProvider {
    type Vtable = AsyncIFtpRoleProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1048821657,
        28908,
        16842,
        [132, 182, 172, 167, 199, 166, 44, 175],
    );
}
impl ::std::convert::From<AsyncIFtpRoleProvider> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIFtpRoleProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIFtpRoleProvider> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIFtpRoleProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIFtpRoleProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIFtpRoleProvider
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
pub struct AsyncIFtpRoleProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        pszrole: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisinrole: *mut super::super::Foundation::BOOL,
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
pub struct AsyncIMSAdminBaseSinkW(::windows::runtime::IUnknown);
impl AsyncIMSAdminBaseSinkW {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_SinkNotify(
        &self,
        dwmdnumelements: u32,
        pcochangelist: *const MD_CHANGE_OBJECT_W,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwmdnumelements),
            ::std::mem::transmute(pcochangelist),
        )
        .ok()
    }
    pub unsafe fn Finish_SinkNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Begin_ShutdownNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Finish_ShutdownNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIMSAdminBaseSinkW {
    type Vtable = AsyncIMSAdminBaseSinkW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2850461203,
        47117,
        4560,
        [185, 185, 0, 160, 201, 34, 231, 80],
    );
}
impl ::std::convert::From<AsyncIMSAdminBaseSinkW> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIMSAdminBaseSinkW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIMSAdminBaseSinkW> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIMSAdminBaseSinkW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AsyncIMSAdminBaseSinkW
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AsyncIMSAdminBaseSinkW
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
pub struct AsyncIMSAdminBaseSinkW_abi(
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
        dwmdnumelements: u32,
        pcochangelist: *const MD_CHANGE_OBJECT_W,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::super::Security::Cryptography::Core::CERT_CONTEXT,
    pub cbAllocated: u32,
    pub dwCertificateFlags: u32,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CERT_CONTEXT_EX {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CERT_CONTEXT_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CERT_CONTEXT_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CERT_CONTEXT_EX")
            .field("CertContext", &self.CertContext)
            .field("cbAllocated", &self.cbAllocated)
            .field("dwCertificateFlags", &self.dwCertificateFlags)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CERT_CONTEXT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.CertContext == other.CertContext
            && self.cbAllocated == other.cbAllocated
            && self.dwCertificateFlags == other.dwCertificateFlags
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CERT_CONTEXT_EX {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CERT_CONTEXT_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLSID_IImgCtx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    810611670,
    39093,
    4559,
    [187, 130, 0, 170, 0, 189, 206, 11],
);
pub const CLSID_IisServiceControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3908797985,
    22671,
    4562,
    [157, 97, 0, 192, 79, 121, 197, 254],
);
pub const CLSID_MSAdminBase_W: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2850461200,
    47117,
    4560,
    [185, 185, 0, 160, 201, 34, 231, 80],
);
pub const CLSID_Request: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2450269648,
    9689,
    4560,
    [165, 95, 0, 160, 201, 12, 32, 145],
);
pub const CLSID_Response: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1189190560,
    9693,
    4560,
    [165, 95, 0, 160, 201, 12, 32, 145],
);
pub const CLSID_ScriptingContext: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3648679328,
    43112,
    4559,
    [131, 174, 17, 176, 201, 12, 43, 216],
);
pub const CLSID_Server: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2768687456,
    9696,
    4560,
    [165, 95, 0, 160, 201, 12, 32, 145],
);
pub const CLSID_Session: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1352634144,
    9694,
    4560,
    [165, 95, 0, 160, 201, 12, 32, 145],
);
pub const CLSID_WamAdmin: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1634960964,
    61846,
    4560,
    [153, 83, 0, 192, 79, 217, 25, 193],
);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIGURATION_ENTRY {
    pub bstrKey: super::super::Foundation::BSTR,
    pub bstrValue: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CONFIGURATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONFIGURATION_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONFIGURATION_ENTRY")
            .field("bstrKey", &self.bstrKey)
            .field("bstrValue", &self.bstrValue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONFIGURATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.bstrKey == other.bstrKey && self.bstrValue == other.bstrValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONFIGURATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONFIGURATION_ENTRY {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const DISPID_HTTPREQUEST_ABORT: u32 = 12u32;
pub const DISPID_HTTPREQUEST_BASE: u32 = 1u32;
pub const DISPID_HTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 4u32;
pub const DISPID_HTTPREQUEST_GETRESPONSEHEADER: u32 = 3u32;
pub const DISPID_HTTPREQUEST_OPEN: u32 = 1u32;
pub const DISPID_HTTPREQUEST_OPTION: u32 = 6u32;
pub const DISPID_HTTPREQUEST_RESPONSEBODY: u32 = 10u32;
pub const DISPID_HTTPREQUEST_RESPONSESTREAM: u32 = 11u32;
pub const DISPID_HTTPREQUEST_RESPONSETEXT: u32 = 9u32;
pub const DISPID_HTTPREQUEST_SEND: u32 = 5u32;
pub const DISPID_HTTPREQUEST_SETAUTOLOGONPOLICY: u32 = 18u32;
pub const DISPID_HTTPREQUEST_SETCLIENTCERTIFICATE: u32 = 17u32;
pub const DISPID_HTTPREQUEST_SETCREDENTIALS: u32 = 14u32;
pub const DISPID_HTTPREQUEST_SETPROXY: u32 = 13u32;
pub const DISPID_HTTPREQUEST_SETREQUESTHEADER: u32 = 2u32;
pub const DISPID_HTTPREQUEST_SETTIMEOUTS: u32 = 16u32;
pub const DISPID_HTTPREQUEST_STATUS: u32 = 7u32;
pub const DISPID_HTTPREQUEST_STATUSTEXT: u32 = 8u32;
pub const DISPID_HTTPREQUEST_WAITFORRESPONSE: u32 = 15u32;
pub const DWN_COLORMODE: u32 = 63u32;
pub const DWN_DOWNLOADONLY: u32 = 64u32;
pub const DWN_FORCEDITHER: u32 = 128u32;
pub const DWN_MIRRORIMAGE: u32 = 512u32;
pub const DWN_RAWIMAGE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: *mut ::std::ffi::c_void,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [super::SystemServices::CHAR; 80],
    pub lpszMethod: super::super::Foundation::PSTR,
    pub lpszQueryString: super::super::Foundation::PSTR,
    pub lpszPathInfo: super::super::Foundation::PSTR,
    pub lpszPathTranslated: super::super::Foundation::PSTR,
    pub cbTotalBytes: u32,
    pub cbAvailable: u32,
    pub lpbData: *mut u8,
    pub lpszContentType: super::super::Foundation::PSTR,
    pub GetServerVariable: isize,
    pub WriteClient: isize,
    pub ReadClient: isize,
    pub ServerSupportFunction: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl EXTENSION_CONTROL_BLOCK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for EXTENSION_CONTROL_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EXTENSION_CONTROL_BLOCK")
            .field("cbSize", &self.cbSize)
            .field("dwVersion", &self.dwVersion)
            .field("ConnID", &self.ConnID)
            .field("dwHttpStatusCode", &self.dwHttpStatusCode)
            .field("lpszLogData", &self.lpszLogData)
            .field("lpszMethod", &self.lpszMethod)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("lpszPathInfo", &self.lpszPathInfo)
            .field("lpszPathTranslated", &self.lpszPathTranslated)
            .field("cbTotalBytes", &self.cbTotalBytes)
            .field("cbAvailable", &self.cbAvailable)
            .field("lpbData", &self.lpbData)
            .field("lpszContentType", &self.lpszContentType)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("WriteClient", &self.WriteClient)
            .field("ReadClient", &self.ReadClient)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for EXTENSION_CONTROL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwVersion == other.dwVersion
            && self.ConnID == other.ConnID
            && self.dwHttpStatusCode == other.dwHttpStatusCode
            && self.lpszLogData == other.lpszLogData
            && self.lpszMethod == other.lpszMethod
            && self.lpszQueryString == other.lpszQueryString
            && self.lpszPathInfo == other.lpszPathInfo
            && self.lpszPathTranslated == other.lpszPathTranslated
            && self.cbTotalBytes == other.cbTotalBytes
            && self.cbAvailable == other.cbAvailable
            && self.lpbData == other.lpbData
            && self.lpszContentType == other.lpszContentType
            && self.GetServerVariable == other.GetServerVariable
            && self.WriteClient == other.WriteClient
            && self.ReadClient == other.ReadClient
            && self.ServerSupportFunction == other.ServerSupportFunction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for EXTENSION_CONTROL_BLOCK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for EXTENSION_CONTROL_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FP_MD_ID_BEGIN_RESERVED: u32 = 32768u32;
pub const FP_MD_ID_END_RESERVED: u32 = 36863u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FTP_ACCESS(pub i32);
pub const FTP_ACCESS_NONE: FTP_ACCESS = FTP_ACCESS(0i32);
pub const FTP_ACCESS_READ: FTP_ACCESS = FTP_ACCESS(1i32);
pub const FTP_ACCESS_WRITE: FTP_ACCESS = FTP_ACCESS(2i32);
pub const FTP_ACCESS_READ_WRITE: FTP_ACCESS = FTP_ACCESS(3i32);
impl ::std::convert::From<i32> for FTP_ACCESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FTP_ACCESS {
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
pub struct FTP_PROCESS_STATUS(pub i32);
pub const FTP_PROCESS_CONTINUE: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(0i32);
pub const FTP_PROCESS_CLOSE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(1i32);
pub const FTP_PROCESS_TERMINATE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(2i32);
pub const FTP_PROCESS_REJECT_COMMAND: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(3i32);
impl ::std::convert::From<i32> for FTP_PROCESS_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FTP_PROCESS_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FtpProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1891485287,
    13234,
    17904,
    [172, 82, 195, 202, 70, 247, 166, 86],
);
pub const GUID_IIS_ALL_TRACE_PROVIDERS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const GUID_IIS_ASPNET_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2951774718,
        583,
        17013,
        [156, 78, 2, 31, 61, 193, 218, 53],
    );
pub const GUID_IIS_ASP_TRACE_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        112807322,
        45406,
        17774,
        [164, 239, 55, 201, 132, 162, 203, 75],
    );
pub const GUID_IIS_ISAPI_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2713846798,
        34880,
        19505,
        [186, 17, 152, 113, 3, 26, 25, 234],
    );
pub const GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3579657161,
        52137,
        17631,
        [130, 126, 19, 45, 58, 69, 150, 194],
    );
pub const GUID_IIS_WWW_SERVER_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        975851140,
        19489,
        18817,
        [174, 16, 63, 218, 13, 155, 15, 131],
    );
pub const GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3729148361,
        5608,
        20458,
        [157, 133, 28, 221, 165, 32, 195, 52],
    );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetExtensionVersion(::std::mem::transmute(pver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFilterVersion(::std::mem::transmute(pver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003u32;
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0u32;
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1u32;
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: super::super::Foundation::PSTR,
    pub uHttpSubError: u16,
    pub fAsync: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_CUSTOM_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_CUSTOM_ERROR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_CUSTOM_ERROR_INFO")
            .field("pszStatus", &self.pszStatus)
            .field("uHttpSubError", &self.uHttpSubError)
            .field("fAsync", &self.fAsync)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_CUSTOM_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus
            && self.uHttpSubError == other.uHttpSubError
            && self.fAsync == other.fAsync
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_CUSTOM_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_CUSTOM_ERROR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: super::super::Foundation::PWSTR,
    pub pszMethod: super::super::Foundation::PSTR,
    pub pszChildHeaders: super::super::Foundation::PSTR,
    pub pUserInfo: *mut HSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_EXEC_UNICODE_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_EXEC_UNICODE_URL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_EXEC_UNICODE_URL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_UNICODE_URL_INFO")
            .field("pszUrl", &self.pszUrl)
            .field("pszMethod", &self.pszMethod)
            .field("pszChildHeaders", &self.pszChildHeaders)
            .field("pUserInfo", &self.pUserInfo)
            .field("pEntity", &self.pEntity)
            .field("dwExecUrlFlags", &self.dwExecUrlFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_EXEC_UNICODE_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl
            && self.pszMethod == other.pszMethod
            && self.pszChildHeaders == other.pszChildHeaders
            && self.pUserInfo == other.pUserInfo
            && self.pEntity == other.pEntity
            && self.dwExecUrlFlags == other.dwExecUrlFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_EXEC_UNICODE_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_EXEC_UNICODE_URL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: super::super::Foundation::PWSTR,
    pub pszCustomAuthType: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_EXEC_UNICODE_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_UNICODE_URL_USER_INFO")
            .field("hImpersonationToken", &self.hImpersonationToken)
            .field("pszCustomUserName", &self.pszCustomUserName)
            .field("pszCustomAuthType", &self.pszCustomAuthType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken
            && self.pszCustomUserName == other.pszCustomUserName
            && self.pszCustomAuthType == other.pszCustomAuthType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_EXEC_UNICODE_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_EXEC_UNICODE_URL_USER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HSE_EXEC_URL_ENTITY_INFO {
    pub cbAvailable: u32,
    pub lpbData: *mut ::std::ffi::c_void,
}
impl HSE_EXEC_URL_ENTITY_INFO {}
impl ::std::default::Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HSE_EXEC_URL_ENTITY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_URL_ENTITY_INFO")
            .field("cbAvailable", &self.cbAvailable)
            .field("lpbData", &self.lpbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HSE_EXEC_URL_ENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbAvailable == other.cbAvailable && self.lpbData == other.lpbData
    }
}
impl ::std::cmp::Eq for HSE_EXEC_URL_ENTITY_INFO {}
unsafe impl ::windows::runtime::Abi for HSE_EXEC_URL_ENTITY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: u32 = 128u32;
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: u32 = 4u32;
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: super::super::Foundation::PSTR,
    pub pszMethod: super::super::Foundation::PSTR,
    pub pszChildHeaders: super::super::Foundation::PSTR,
    pub pUserInfo: *mut HSE_EXEC_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_EXEC_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_EXEC_URL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_EXEC_URL_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_URL_INFO")
            .field("pszUrl", &self.pszUrl)
            .field("pszMethod", &self.pszMethod)
            .field("pszChildHeaders", &self.pszChildHeaders)
            .field("pUserInfo", &self.pUserInfo)
            .field("pEntity", &self.pEntity)
            .field("dwExecUrlFlags", &self.dwExecUrlFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_EXEC_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl
            && self.pszMethod == other.pszMethod
            && self.pszChildHeaders == other.pszChildHeaders
            && self.pUserInfo == other.pUserInfo
            && self.pEntity == other.pEntity
            && self.dwExecUrlFlags == other.dwExecUrlFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_EXEC_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_EXEC_URL_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2u32;
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: u16,
    pub uHttpSubStatus: u16,
    pub dwWin32Error: u32,
}
impl HSE_EXEC_URL_STATUS {}
impl ::std::default::Default for HSE_EXEC_URL_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HSE_EXEC_URL_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_URL_STATUS")
            .field("uHttpStatusCode", &self.uHttpStatusCode)
            .field("uHttpSubStatus", &self.uHttpSubStatus)
            .field("dwWin32Error", &self.dwWin32Error)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HSE_EXEC_URL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uHttpStatusCode == other.uHttpStatusCode
            && self.uHttpSubStatus == other.uHttpSubStatus
            && self.dwWin32Error == other.dwWin32Error
    }
}
impl ::std::cmp::Eq for HSE_EXEC_URL_STATUS {}
unsafe impl ::windows::runtime::Abi for HSE_EXEC_URL_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: super::super::Foundation::PSTR,
    pub pszCustomAuthType: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_EXEC_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_EXEC_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_EXEC_URL_USER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_EXEC_URL_USER_INFO")
            .field("hImpersonationToken", &self.hImpersonationToken)
            .field("pszCustomUserName", &self.pszCustomUserName)
            .field("pszCustomAuthType", &self.pszCustomAuthType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_EXEC_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken
            && self.pszCustomUserName == other.pszCustomUserName
            && self.pszCustomAuthType == other.pszCustomAuthType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_EXEC_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_EXEC_URL_USER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_LOG_BUFFER_LEN: u32 = 80u32;
pub const HSE_MAX_EXT_DLL_NAME_LEN: u32 = 256u32;
pub const HSE_REQ_ABORTIVE_CLOSE: u32 = 1014u32;
pub const HSE_REQ_ASYNC_READ_CLIENT: u32 = 1010u32;
pub const HSE_REQ_BASE: u32 = 0u32;
pub const HSE_REQ_CANCEL_IO: u32 = 1049u32;
pub const HSE_REQ_CLOSE_CONNECTION: u32 = 1017u32;
pub const HSE_REQ_DONE_WITH_SESSION: u32 = 4u32;
pub const HSE_REQ_END_RESERVED: u32 = 1000u32;
pub const HSE_REQ_EXEC_UNICODE_URL: u32 = 1025u32;
pub const HSE_REQ_EXEC_URL: u32 = 1026u32;
pub const HSE_REQ_GET_ANONYMOUS_TOKEN: u32 = 1038u32;
pub const HSE_REQ_GET_CACHE_INVALIDATION_CALLBACK: u32 = 1040u32;
pub const HSE_REQ_GET_CERT_INFO_EX: u32 = 1015u32;
pub const HSE_REQ_GET_CHANNEL_BINDING_TOKEN: u32 = 1050u32;
pub const HSE_REQ_GET_CONFIG_OBJECT: u32 = 1046u32;
pub const HSE_REQ_GET_EXEC_URL_STATUS: u32 = 1027u32;
pub const HSE_REQ_GET_IMPERSONATION_TOKEN: u32 = 1011u32;
pub const HSE_REQ_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK: u32 = 1048u32;
pub const HSE_REQ_GET_SSPI_INFO: u32 = 1002u32;
pub const HSE_REQ_GET_TRACE_INFO: u32 = 1042u32;
pub const HSE_REQ_GET_TRACE_INFO_EX: u32 = 1044u32;
pub const HSE_REQ_GET_UNICODE_ANONYMOUS_TOKEN: u32 = 1041u32;
pub const HSE_REQ_GET_WORKER_PROCESS_SETTINGS: u32 = 1047u32;
pub const HSE_REQ_IO_COMPLETION: u32 = 1005u32;
pub const HSE_REQ_IS_CONNECTED: u32 = 1018u32;
pub const HSE_REQ_IS_IN_PROCESS: u32 = 1030u32;
pub const HSE_REQ_IS_KEEP_CONN: u32 = 1008u32;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH: u32 = 1023u32;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH_EX: u32 = 1024u32;
pub const HSE_REQ_MAP_URL_TO_PATH: u32 = 1001u32;
pub const HSE_REQ_MAP_URL_TO_PATH_EX: u32 = 1012u32;
pub const HSE_REQ_NORMALIZE_URL: u32 = 1033u32;
pub const HSE_REQ_RAISE_TRACE_EVENT: u32 = 1045u32;
pub const HSE_REQ_REFRESH_ISAPI_ACL: u32 = 1007u32;
pub const HSE_REQ_REPORT_UNHEALTHY: u32 = 1032u32;
pub const HSE_REQ_SEND_CUSTOM_ERROR: u32 = 1028u32;
pub const HSE_REQ_SEND_RESPONSE_HEADER: u32 = 3u32;
pub const HSE_REQ_SEND_RESPONSE_HEADER_EX: u32 = 1016u32;
pub const HSE_REQ_SEND_URL: u32 = 2u32;
pub const HSE_REQ_SEND_URL_REDIRECT_RESP: u32 = 1u32;
pub const HSE_REQ_SET_FLUSH_FLAG: u32 = 1043u32;
pub const HSE_REQ_TRANSMIT_FILE: u32 = 1006u32;
pub const HSE_REQ_VECTOR_SEND: u32 = 1037u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: u32,
    pub pszStatus: super::super::Foundation::PSTR,
    pub pszHeaders: super::super::Foundation::PSTR,
    pub nElementCount: u32,
    pub lpElementArray: *mut HSE_VECTOR_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_RESPONSE_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_RESPONSE_VECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_RESPONSE_VECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_RESPONSE_VECTOR")
            .field("dwFlags", &self.dwFlags)
            .field("pszStatus", &self.pszStatus)
            .field("pszHeaders", &self.pszHeaders)
            .field("nElementCount", &self.nElementCount)
            .field("lpElementArray", &self.lpElementArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_RESPONSE_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.pszStatus == other.pszStatus
            && self.pszHeaders == other.pszHeaders
            && self.nElementCount == other.nElementCount
            && self.lpElementArray == other.lpElementArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_RESPONSE_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_RESPONSE_VECTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: super::super::Foundation::PSTR,
    pub pszHeader: super::super::Foundation::PSTR,
    pub cchStatus: u32,
    pub cchHeader: u32,
    pub fKeepConn: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_SEND_HEADER_EX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_SEND_HEADER_EX_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_SEND_HEADER_EX_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_SEND_HEADER_EX_INFO")
            .field("pszStatus", &self.pszStatus)
            .field("pszHeader", &self.pszHeader)
            .field("cchStatus", &self.cchStatus)
            .field("cchHeader", &self.cchHeader)
            .field("fKeepConn", &self.fKeepConn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_SEND_HEADER_EX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus
            && self.pszHeader == other.pszHeader
            && self.cchStatus == other.cchStatus
            && self.cchHeader == other.cchHeader
            && self.fKeepConn == other.fKeepConn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_SEND_HEADER_EX_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_SEND_HEADER_EX_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_STATUS_ERROR: u32 = 4u32;
pub const HSE_STATUS_PENDING: u32 = 3u32;
pub const HSE_STATUS_SUCCESS: u32 = 1u32;
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: u32 = 2u32;
pub const HSE_TERM_ADVISORY_UNLOAD: u32 = 1u32;
pub const HSE_TERM_MUST_UNLOAD: u32 = 2u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct HSE_TF_INFO {
    pub pfnHseIO: ::std::option::Option<PFN_HSE_IO_COMPLETION>,
    pub pContext: *mut ::std::ffi::c_void,
    pub hFile: super::super::Foundation::HANDLE,
    pub pszStatusCode: super::super::Foundation::PSTR,
    pub BytesToWrite: u32,
    pub Offset: u32,
    pub pHead: *mut ::std::ffi::c_void,
    pub HeadLength: u32,
    pub pTail: *mut ::std::ffi::c_void,
    pub TailLength: u32,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl HSE_TF_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for HSE_TF_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_TF_INFO")
            .field("pContext", &self.pContext)
            .field("hFile", &self.hFile)
            .field("pszStatusCode", &self.pszStatusCode)
            .field("BytesToWrite", &self.BytesToWrite)
            .field("Offset", &self.Offset)
            .field("pHead", &self.pHead)
            .field("HeadLength", &self.HeadLength)
            .field("pTail", &self.pTail)
            .field("TailLength", &self.TailLength)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for HSE_TF_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pfnHseIO.map(|f| f as usize) == other.pfnHseIO.map(|f| f as usize)
            && self.pContext == other.pContext
            && self.hFile == other.hFile
            && self.pszStatusCode == other.pszStatusCode
            && self.BytesToWrite == other.BytesToWrite
            && self.Offset == other.Offset
            && self.pHead == other.pHead
            && self.HeadLength == other.HeadLength
            && self.pTail == other.pTail
            && self.TailLength == other.TailLength
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for HSE_TF_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for HSE_TF_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: super::super::Foundation::BOOL,
    pub TraceContextId: [u8; 16],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HSE_TRACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HSE_TRACE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_TRACE_INFO")
            .field("fTraceRequest", &self.fTraceRequest)
            .field("TraceContextId", &self.TraceContextId)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSE_TRACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fTraceRequest == other.fTraceRequest
            && self.TraceContextId == other.TraceContextId
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSE_TRACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSE_TRACE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HSE_UNICODE_URL_MAPEX_INFO {
    pub lpszPath: [u16; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
}
impl HSE_UNICODE_URL_MAPEX_INFO {}
impl ::std::default::Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HSE_UNICODE_URL_MAPEX_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_UNICODE_URL_MAPEX_INFO")
            .field("lpszPath", &self.lpszPath)
            .field("dwFlags", &self.dwFlags)
            .field("cchMatchingPath", &self.cchMatchingPath)
            .field("cchMatchingURL", &self.cchMatchingURL)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HSE_UNICODE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath
            && self.dwFlags == other.dwFlags
            && self.cchMatchingPath == other.cchMatchingPath
            && self.cchMatchingURL == other.cchMatchingURL
    }
}
impl ::std::cmp::Eq for HSE_UNICODE_URL_MAPEX_INFO {}
unsafe impl ::windows::runtime::Abi for HSE_UNICODE_URL_MAPEX_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_URL_FLAGS_DONT_CACHE: u32 = 16u32;
pub const HSE_URL_FLAGS_EXECUTE: u32 = 4u32;
pub const HSE_URL_FLAGS_MAP_CERT: u32 = 128u32;
pub const HSE_URL_FLAGS_MASK: u32 = 1023u32;
pub const HSE_URL_FLAGS_NEGO_CERT: u32 = 32u32;
pub const HSE_URL_FLAGS_READ: u32 = 1u32;
pub const HSE_URL_FLAGS_REQUIRE_CERT: u32 = 64u32;
pub const HSE_URL_FLAGS_SCRIPT: u32 = 512u32;
pub const HSE_URL_FLAGS_SSL: u32 = 8u32;
pub const HSE_URL_FLAGS_SSL128: u32 = 256u32;
pub const HSE_URL_FLAGS_WRITE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [super::SystemServices::CHAR; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl HSE_URL_MAPEX_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for HSE_URL_MAPEX_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_URL_MAPEX_INFO")
            .field("lpszPath", &self.lpszPath)
            .field("dwFlags", &self.dwFlags)
            .field("cchMatchingPath", &self.cchMatchingPath)
            .field("cchMatchingURL", &self.cchMatchingURL)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for HSE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath
            && self.dwFlags == other.dwFlags
            && self.cchMatchingPath == other.cchMatchingPath
            && self.cchMatchingURL == other.cchMatchingURL
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for HSE_URL_MAPEX_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for HSE_URL_MAPEX_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HSE_VECTOR_ELEMENT {
    pub ElementType: u32,
    pub pvContext: *mut ::std::ffi::c_void,
    pub cbOffset: u64,
    pub cbSize: u64,
}
impl HSE_VECTOR_ELEMENT {}
impl ::std::default::Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HSE_VECTOR_ELEMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_VECTOR_ELEMENT")
            .field("ElementType", &self.ElementType)
            .field("pvContext", &self.pvContext)
            .field("cbOffset", &self.cbOffset)
            .field("cbSize", &self.cbSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HSE_VECTOR_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType
            && self.pvContext == other.pvContext
            && self.cbOffset == other.cbOffset
            && self.cbSize == other.cbSize
    }
}
impl ::std::cmp::Eq for HSE_VECTOR_ELEMENT {}
unsafe impl ::windows::runtime::Abi for HSE_VECTOR_ELEMENT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: u32 = 1u32;
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: u32,
    pub lpszExtensionDesc: [super::SystemServices::CHAR; 256],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl HSE_VERSION_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for HSE_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HSE_VERSION_INFO")
            .field("dwExtensionVersion", &self.dwExtensionVersion)
            .field("lpszExtensionDesc", &self.lpszExtensionDesc)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for HSE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExtensionVersion == other.dwExtensionVersion
            && self.lpszExtensionDesc == other.lpszExtensionDesc
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for HSE_VERSION_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for HSE_VERSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HSE_VERSION_MAJOR: u32 = 8u32;
pub const HSE_VERSION_MINOR: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: super::super::Foundation::PSTR,
    pub pszPhysicalPath: super::super::Foundation::PSTR,
    pub dwReason: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_ACCESS_DENIED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_ACCESS_DENIED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_ACCESS_DENIED")
            .field("pszURL", &self.pszURL)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("dwReason", &self.dwReason)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_ACCESS_DENIED {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.dwReason == other.dwReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_ACCESS_DENIED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_ACCESS_DENIED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: super::super::Foundation::PSTR,
    pub cbUserBuff: u32,
    pub pszPassword: super::super::Foundation::PSTR,
    pub cbPasswordBuff: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_AUTHENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_AUTHENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_AUTHENT")
            .field("pszUser", &self.pszUser)
            .field("cbUserBuff", &self.cbUserBuff)
            .field("pszPassword", &self.pszPassword)
            .field("cbPasswordBuff", &self.cbPasswordBuff)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_AUTHENT {
    fn eq(&self, other: &Self) -> bool {
        self.pszUser == other.pszUser
            && self.cbUserBuff == other.cbUserBuff
            && self.pszPassword == other.pszPassword
            && self.cbPasswordBuff == other.cbPasswordBuff
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_AUTHENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_AUTHENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub GetUserToken: isize,
    pub HttpStatus: u32,
    pub fResetAuth: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_AUTH_COMPLETE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_AUTH_COMPLETE_INFO")
            .field("GetHeader", &self.GetHeader)
            .field("SetHeader", &self.SetHeader)
            .field("AddHeader", &self.AddHeader)
            .field("GetUserToken", &self.GetUserToken)
            .field("HttpStatus", &self.HttpStatus)
            .field("fResetAuth", &self.fResetAuth)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader
            && self.SetHeader == other.SetHeader
            && self.AddHeader == other.AddHeader
            && self.GetUserToken == other.GetUserToken
            && self.HttpStatus == other.HttpStatus
            && self.fResetAuth == other.fResetAuth
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_AUTH_COMPLETE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_AUTH_COMPLETE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: u32,
    pub Revision: u32,
    pub ServerContext: *mut ::std::ffi::c_void,
    pub ulReserved: u32,
    pub fIsSecurePort: super::super::Foundation::BOOL,
    pub pFilterContext: *mut ::std::ffi::c_void,
    pub GetServerVariable: isize,
    pub AddResponseHeaders: isize,
    pub WriteClient: isize,
    pub AllocMem: isize,
    pub ServerSupportFunction: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_CONTEXT")
            .field("cbSize", &self.cbSize)
            .field("Revision", &self.Revision)
            .field("ServerContext", &self.ServerContext)
            .field("ulReserved", &self.ulReserved)
            .field("fIsSecurePort", &self.fIsSecurePort)
            .field("pFilterContext", &self.pFilterContext)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("AddResponseHeaders", &self.AddResponseHeaders)
            .field("WriteClient", &self.WriteClient)
            .field("AllocMem", &self.AllocMem)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.Revision == other.Revision
            && self.ServerContext == other.ServerContext
            && self.ulReserved == other.ulReserved
            && self.fIsSecurePort == other.fIsSecurePort
            && self.pFilterContext == other.pFilterContext
            && self.GetServerVariable == other.GetServerVariable
            && self.AddResponseHeaders == other.AddResponseHeaders
            && self.WriteClient == other.WriteClient
            && self.AllocMem == other.AllocMem
            && self.ServerSupportFunction == other.ServerSupportFunction
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: super::super::Foundation::PSTR,
    pub pszClientUserName: super::super::Foundation::PSTR,
    pub pszServerName: super::super::Foundation::PSTR,
    pub pszOperation: super::super::Foundation::PSTR,
    pub pszTarget: super::super::Foundation::PSTR,
    pub pszParameters: super::super::Foundation::PSTR,
    pub dwHttpStatus: u32,
    pub dwWin32Status: u32,
    pub dwBytesSent: u32,
    pub dwBytesRecvd: u32,
    pub msTimeForProcessing: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_LOG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_LOG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_LOG")
            .field("pszClientHostName", &self.pszClientHostName)
            .field("pszClientUserName", &self.pszClientUserName)
            .field("pszServerName", &self.pszServerName)
            .field("pszOperation", &self.pszOperation)
            .field("pszTarget", &self.pszTarget)
            .field("pszParameters", &self.pszParameters)
            .field("dwHttpStatus", &self.dwHttpStatus)
            .field("dwWin32Status", &self.dwWin32Status)
            .field("dwBytesSent", &self.dwBytesSent)
            .field("dwBytesRecvd", &self.dwBytesRecvd)
            .field("msTimeForProcessing", &self.msTimeForProcessing)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_LOG {
    fn eq(&self, other: &Self) -> bool {
        self.pszClientHostName == other.pszClientHostName
            && self.pszClientUserName == other.pszClientUserName
            && self.pszServerName == other.pszServerName
            && self.pszOperation == other.pszOperation
            && self.pszTarget == other.pszTarget
            && self.pszParameters == other.pszParameters
            && self.dwHttpStatus == other.dwHttpStatus
            && self.dwWin32Status == other.dwWin32Status
            && self.dwBytesSent == other.dwBytesSent
            && self.dwBytesRecvd == other.dwBytesRecvd
            && self.msTimeForProcessing == other.msTimeForProcessing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_LOG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_LOG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub HttpStatus: u32,
    pub dwReserved: u32,
}
impl HTTP_FILTER_PREPROC_HEADERS {}
impl ::std::default::Default for HTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_FILTER_PREPROC_HEADERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_PREPROC_HEADERS")
            .field("GetHeader", &self.GetHeader)
            .field("SetHeader", &self.SetHeader)
            .field("AddHeader", &self.AddHeader)
            .field("HttpStatus", &self.HttpStatus)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_FILTER_PREPROC_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader
            && self.SetHeader == other.SetHeader
            && self.AddHeader == other.AddHeader
            && self.HttpStatus == other.HttpStatus
            && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for HTTP_FILTER_PREPROC_HEADERS {}
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_PREPROC_HEADERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HTTP_FILTER_RAW_DATA {
    pub pvInData: *mut ::std::ffi::c_void,
    pub cbInData: u32,
    pub cbInBuffer: u32,
    pub dwReserved: u32,
}
impl HTTP_FILTER_RAW_DATA {}
impl ::std::default::Default for HTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HTTP_FILTER_RAW_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_RAW_DATA")
            .field("pvInData", &self.pvInData)
            .field("cbInData", &self.cbInData)
            .field("cbInBuffer", &self.cbInBuffer)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HTTP_FILTER_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pvInData == other.pvInData
            && self.cbInData == other.cbInData
            && self.cbInBuffer == other.cbInBuffer
            && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for HTTP_FILTER_RAW_DATA {}
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_RAW_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: super::super::Foundation::PSTR,
    pub pszPhysicalPath: super::super::Foundation::PSTR,
    pub cbPathBuff: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_URL_MAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_URL_MAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_URL_MAP")
            .field("pszURL", &self.pszURL)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("cbPathBuff", &self.cbPathBuff)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_URL_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.cbPathBuff == other.cbPathBuff
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_URL_MAP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_URL_MAP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: super::super::Foundation::PSTR,
    pub pszPhysicalPath: super::super::Foundation::PSTR,
    pub cbPathBuff: u32,
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub pszScriptMapEntry: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_FILTER_URL_MAP_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_FILTER_URL_MAP_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_URL_MAP_EX")
            .field("pszURL", &self.pszURL)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("cbPathBuff", &self.cbPathBuff)
            .field("dwFlags", &self.dwFlags)
            .field("cchMatchingPath", &self.cchMatchingPath)
            .field("cchMatchingURL", &self.cchMatchingURL)
            .field("pszScriptMapEntry", &self.pszScriptMapEntry)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_FILTER_URL_MAP_EX {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.cbPathBuff == other.cbPathBuff
            && self.dwFlags == other.dwFlags
            && self.cchMatchingPath == other.cchMatchingPath
            && self.cchMatchingURL == other.cchMatchingURL
            && self.pszScriptMapEntry == other.pszScriptMapEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_FILTER_URL_MAP_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_URL_MAP_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: u32,
    pub dwFilterVersion: u32,
    pub lpszFilterDesc: [super::SystemServices::CHAR; 257],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl HTTP_FILTER_VERSION {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for HTTP_FILTER_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_FILTER_VERSION")
            .field("dwServerFilterVersion", &self.dwServerFilterVersion)
            .field("dwFilterVersion", &self.dwFilterVersion)
            .field("lpszFilterDesc", &self.lpszFilterDesc)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for HTTP_FILTER_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwServerFilterVersion == other.dwServerFilterVersion
            && self.dwFilterVersion == other.dwFilterVersion
            && self.lpszFilterDesc == other.lpszFilterDesc
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for HTTP_FILTER_VERSION {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for HTTP_FILTER_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_TRACE_CONFIGURATION {
    pub pProviderGuid: *mut ::windows::runtime::GUID,
    pub dwAreas: u32,
    pub dwVerbosity: u32,
    pub fProviderEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_TRACE_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_TRACE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_TRACE_CONFIGURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TRACE_CONFIGURATION")
            .field("pProviderGuid", &self.pProviderGuid)
            .field("dwAreas", &self.dwAreas)
            .field("dwVerbosity", &self.dwVerbosity)
            .field("fProviderEnabled", &self.fProviderEnabled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_TRACE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid
            && self.dwAreas == other.dwAreas
            && self.dwVerbosity == other.dwVerbosity
            && self.fProviderEnabled == other.fProviderEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_TRACE_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_TRACE_CONFIGURATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_TRACE_EVENT {
    pub pProviderGuid: *mut ::windows::runtime::GUID,
    pub dwArea: u32,
    pub pAreaGuid: *mut ::windows::runtime::GUID,
    pub dwEvent: u32,
    pub pszEventName: super::super::Foundation::PWSTR,
    pub dwEventVersion: u32,
    pub dwVerbosity: u32,
    pub pActivityGuid: *mut ::windows::runtime::GUID,
    pub pRelatedActivityGuid: *mut ::windows::runtime::GUID,
    pub dwTimeStamp: u32,
    pub dwFlags: u32,
    pub cEventItems: u32,
    pub pEventItems: *mut HTTP_TRACE_EVENT_ITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_TRACE_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_TRACE_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TRACE_EVENT")
            .field("pProviderGuid", &self.pProviderGuid)
            .field("dwArea", &self.dwArea)
            .field("pAreaGuid", &self.pAreaGuid)
            .field("dwEvent", &self.dwEvent)
            .field("pszEventName", &self.pszEventName)
            .field("dwEventVersion", &self.dwEventVersion)
            .field("dwVerbosity", &self.dwVerbosity)
            .field("pActivityGuid", &self.pActivityGuid)
            .field("pRelatedActivityGuid", &self.pRelatedActivityGuid)
            .field("dwTimeStamp", &self.dwTimeStamp)
            .field("dwFlags", &self.dwFlags)
            .field("cEventItems", &self.cEventItems)
            .field("pEventItems", &self.pEventItems)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_TRACE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid
            && self.dwArea == other.dwArea
            && self.pAreaGuid == other.pAreaGuid
            && self.dwEvent == other.dwEvent
            && self.pszEventName == other.pszEventName
            && self.dwEventVersion == other.dwEventVersion
            && self.dwVerbosity == other.dwVerbosity
            && self.pActivityGuid == other.pActivityGuid
            && self.pRelatedActivityGuid == other.pRelatedActivityGuid
            && self.dwTimeStamp == other.dwTimeStamp
            && self.dwFlags == other.dwFlags
            && self.cEventItems == other.cEventItems
            && self.pEventItems == other.pEventItems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_TRACE_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_TRACE_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_TRACE_EVENT_ITEM {
    pub pszName: super::super::Foundation::PWSTR,
    pub dwDataType: HTTP_TRACE_TYPE,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pszDataDescription: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_TRACE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTTP_TRACE_EVENT_ITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTTP_TRACE_EVENT_ITEM")
            .field("pszName", &self.pszName)
            .field("dwDataType", &self.dwDataType)
            .field("pbData", &self.pbData)
            .field("cbData", &self.cbData)
            .field("pszDataDescription", &self.pszDataDescription)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTTP_TRACE_EVENT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName
            && self.dwDataType == other.dwDataType
            && self.pbData == other.pbData
            && self.cbData == other.cbData
            && self.pszDataDescription == other.pszDataDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTTP_TRACE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTTP_TRACE_EVENT_ITEM {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HTTP_TRACE_LEVEL_END: u32 = 7u32;
pub const HTTP_TRACE_LEVEL_START: u32 = 6u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HTTP_TRACE_TYPE(pub i32);
pub const HTTP_TRACE_TYPE_BYTE: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(17i32);
pub const HTTP_TRACE_TYPE_USHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(18i32);
pub const HTTP_TRACE_TYPE_ULONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(19i32);
pub const HTTP_TRACE_TYPE_ULONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(21i32);
pub const HTTP_TRACE_TYPE_CHAR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(16i32);
pub const HTTP_TRACE_TYPE_SHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(2i32);
pub const HTTP_TRACE_TYPE_LONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(3i32);
pub const HTTP_TRACE_TYPE_LONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(20i32);
pub const HTTP_TRACE_TYPE_LPCWSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(31i32);
pub const HTTP_TRACE_TYPE_LPCSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(30i32);
pub const HTTP_TRACE_TYPE_LPCGUID: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(72i32);
pub const HTTP_TRACE_TYPE_BOOL: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(11i32);
impl ::std::convert::From<i32> for HTTP_TRACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HTTP_TRACE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32;
        }
        ::std::mem::transmute(HttpExtensionProc(::std::mem::transmute(pecb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HttpFilterProc(
    pfc: *mut HTTP_FILTER_CONTEXT,
    notificationtype: u32,
    pvnotification: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFilterProc(
                pfc: *mut HTTP_FILTER_CONTEXT,
                notificationtype: u32,
                pvnotification: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(HttpFilterProc(
            ::std::mem::transmute(pfc),
            ::std::mem::transmute(notificationtype),
            ::std::mem::transmute(pvnotification),
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
pub struct IADMEXT(::windows::runtime::IUnknown);
impl IADMEXT {
    pub unsafe fn Initialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumDcomCLSIDs(
        &self,
        pclsiddcom: *mut ::windows::runtime::GUID,
        dwenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pclsiddcom),
            ::std::mem::transmute(dwenumindex),
        )
        .ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IADMEXT {
    type Vtable = IADMEXT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1373628784,
        63218,
        4560,
        [185, 189, 0, 160, 201, 34, 231, 80],
    );
}
impl ::std::convert::From<IADMEXT> for ::windows::runtime::IUnknown {
    fn from(value: IADMEXT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IADMEXT> for ::windows::runtime::IUnknown {
    fn from(value: &IADMEXT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IADMEXT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IADMEXT {
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
pub struct IADMEXT_abi(
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
        pclsiddcom: *mut ::windows::runtime::GUID,
        dwenumindex: u32,
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
pub struct IFtpAuthenticationProvider(::windows::runtime::IUnknown);
impl IFtpAuthenticationProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateUser<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
        pszpassword: Param3,
        ppszcanonicalusername: *mut super::super::Foundation::PWSTR,
        pfauthenticated: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
            pszpassword.into_param().abi(),
            ::std::mem::transmute(ppszcanonicalusername),
            ::std::mem::transmute(pfauthenticated),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFtpAuthenticationProvider {
    type Vtable = IFtpAuthenticationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1180301660,
        54696,
        18183,
        [178, 252, 111, 213, 121, 66, 70, 207],
    );
}
impl ::std::convert::From<IFtpAuthenticationProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpAuthenticationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpAuthenticationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpAuthenticationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFtpAuthenticationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpAuthenticationProvider
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
pub struct IFtpAuthenticationProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        pszpassword: super::super::Foundation::PWSTR,
        ppszcanonicalusername: *mut super::super::Foundation::PWSTR,
        pfauthenticated: *mut super::super::Foundation::BOOL,
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
pub struct IFtpAuthorizationProvider(::windows::runtime::IUnknown);
impl IFtpAuthorizationProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserAccessPermission<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszvirtualpath: Param2,
        pszusername: Param3,
    ) -> ::windows::runtime::Result<FTP_ACCESS> {
        let mut result__: <FTP_ACCESS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszvirtualpath.into_param().abi(),
            pszusername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<FTP_ACCESS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFtpAuthorizationProvider {
    type Vtable = IFtpAuthorizationProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2768955297,
        41818,
        17076,
        [164, 243, 244, 247, 5, 122, 5, 209],
    );
}
impl ::std::convert::From<IFtpAuthorizationProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpAuthorizationProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpAuthorizationProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpAuthorizationProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFtpAuthorizationProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpAuthorizationProvider
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
pub struct IFtpAuthorizationProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszvirtualpath: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        pftpaccess: *mut FTP_ACCESS,
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
pub struct IFtpHomeDirectoryProvider(::windows::runtime::IUnknown);
impl IFtpHomeDirectoryProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserHomeDirectoryData<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFtpHomeDirectoryProvider {
    type Vtable = IFtpHomeDirectoryProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        154383250,
        6365,
        16535,
        [139, 156, 131, 50, 92, 53, 217, 166],
    );
}
impl ::std::convert::From<IFtpHomeDirectoryProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpHomeDirectoryProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpHomeDirectoryProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpHomeDirectoryProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFtpHomeDirectoryProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpHomeDirectoryProvider
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
pub struct IFtpHomeDirectoryProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        ppszhomedirectorydata: *mut super::super::Foundation::PWSTR,
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
pub struct IFtpLogProvider(::windows::runtime::IUnknown);
impl IFtpLogProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log(
        &self,
        ploggingparameters: *const LOGGING_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ploggingparameters),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFtpLogProvider {
    type Vtable = IFtpLogProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2710213836,
        33433,
        17416,
        [129, 108, 124, 59, 172, 161, 164, 14],
    );
}
impl ::std::convert::From<IFtpLogProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpLogProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpLogProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpLogProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFtpLogProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFtpLogProvider {
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
pub struct IFtpLogProvider_abi(
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
        ploggingparameters: *const LOGGING_PARAMETERS,
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
pub struct IFtpPostprocessProvider(::windows::runtime::IUnknown);
impl IFtpPostprocessProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePostprocess(
        &self,
        ppostprocessparameters: *const POST_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::Result<FTP_PROCESS_STATUS> {
        let mut result__: <FTP_PROCESS_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppostprocessparameters),
            &mut result__,
        )
        .from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFtpPostprocessProvider {
    type Vtable = IFtpPostprocessProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1159908294,
        5837,
        18861,
        [134, 83, 154, 44, 87, 158, 66, 128],
    );
}
impl ::std::convert::From<IFtpPostprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpPostprocessProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpPostprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpPostprocessProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFtpPostprocessProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpPostprocessProvider
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
pub struct IFtpPostprocessProvider_abi(
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
        ppostprocessparameters: *const POST_PROCESS_PARAMETERS,
        pftpprocessstatus: *mut FTP_PROCESS_STATUS,
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
pub struct IFtpPreprocessProvider(::windows::runtime::IUnknown);
impl IFtpPreprocessProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePreprocess(
        &self,
        ppreprocessparameters: *const PRE_PROCESS_PARAMETERS,
    ) -> ::windows::runtime::Result<FTP_PROCESS_STATUS> {
        let mut result__: <FTP_PROCESS_STATUS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppreprocessparameters),
            &mut result__,
        )
        .from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFtpPreprocessProvider {
    type Vtable = IFtpPreprocessProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2747374432,
        23080,
        18202,
        [143, 147, 171, 48, 65, 28, 238, 130],
    );
}
impl ::std::convert::From<IFtpPreprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpPreprocessProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpPreprocessProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpPreprocessProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFtpPreprocessProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpPreprocessProvider
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
pub struct IFtpPreprocessProvider_abi(
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
        ppreprocessparameters: *const PRE_PROCESS_PARAMETERS,
        pftpprocessstatus: *mut FTP_PROCESS_STATUS,
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
pub struct IFtpProviderConstruct(::windows::runtime::IUnknown);
impl IFtpProviderConstruct {
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn Construct(
        &self,
        configurationentries: *const super::OleAutomation::SAFEARRAY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(configurationentries),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFtpProviderConstruct {
    type Vtable = IFtpProviderConstruct_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1293565819,
        16685,
        17532,
        [177, 153, 100, 249, 103, 233, 162, 218],
    );
}
impl ::std::convert::From<IFtpProviderConstruct> for ::windows::runtime::IUnknown {
    fn from(value: IFtpProviderConstruct) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpProviderConstruct> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpProviderConstruct) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFtpProviderConstruct {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IFtpProviderConstruct
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
pub struct IFtpProviderConstruct_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        configurationentries: *const super::OleAutomation::SAFEARRAY,
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
pub struct IFtpRoleProvider(::windows::runtime::IUnknown);
impl IFtpRoleProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserInRole<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszsessionid: Param0,
        pszsitename: Param1,
        pszusername: Param2,
        pszrole: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszsessionid.into_param().abi(),
            pszsitename.into_param().abi(),
            pszusername.into_param().abi(),
            pszrole.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFtpRoleProvider {
    type Vtable = IFtpRoleProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2426176781,
        36000,
        18036,
        [150, 184, 204, 41, 65, 83, 87, 37],
    );
}
impl ::std::convert::From<IFtpRoleProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFtpRoleProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFtpRoleProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFtpRoleProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFtpRoleProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFtpRoleProvider {
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
pub struct IFtpRoleProvider_abi(
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
        pszsessionid: super::super::Foundation::PWSTR,
        pszsitename: super::super::Foundation::PWSTR,
        pszusername: super::super::Foundation::PWSTR,
        pszrole: super::super::Foundation::PWSTR,
        pfisinrole: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const IIS_MD_ADSI_METAID_BEGIN: u32 = 130000u32;
pub const IIS_MD_APPPOOL_BASE: u32 = 9000u32;
pub const IIS_MD_APP_BASE: u32 = 9100u32;
pub const IIS_MD_FILE_PROP_BASE: u32 = 6000u32;
pub const IIS_MD_FTP_BASE: u32 = 5000u32;
pub const IIS_MD_GLOBAL_BASE: u32 = 9200u32;
pub const IIS_MD_HTTP_BASE: u32 = 2000u32;
pub const IIS_MD_ID_BEGIN_RESERVED: u32 = 1u32;
pub const IIS_MD_ID_END_RESERVED: u32 = 32767u32;
pub const IIS_MD_LOGCUSTOM_BASE: u32 = 4500u32;
pub const IIS_MD_LOGCUSTOM_LAST: u32 = 4508u32;
pub const IIS_MD_LOG_BASE: u32 = 4000u32;
pub const IIS_MD_LOG_LAST: u32 = 4015u32;
pub const IIS_MD_SERVER_BASE: u32 = 1000u32;
pub const IIS_MD_SSL_BASE: u32 = 5500u32;
pub const IIS_MD_UT_END_RESERVED: u32 = 2000u32;
pub const IIS_MD_UT_FILE: u32 = 2u32;
pub const IIS_MD_UT_SERVER: u32 = 1u32;
pub const IIS_MD_UT_WAM: u32 = 100u32;
pub const IIS_MD_VR_BASE: u32 = 3000u32;
pub const IMAP_MD_ID_BEGIN_RESERVED: u32 = 49152u32;
pub const IMAP_MD_ID_END_RESERVED: u32 = 53247u32;
pub const IMGANIM_ANIMATED: u32 = 268435456u32;
pub const IMGANIM_MASK: u32 = 268435456u32;
pub const IMGBITS_MASK: u32 = 234881024u32;
pub const IMGBITS_NONE: u32 = 33554432u32;
pub const IMGBITS_PARTIAL: u32 = 67108864u32;
pub const IMGBITS_TOTAL: u32 = 134217728u32;
pub const IMGCHG_ANIMATE: u32 = 8u32;
pub const IMGCHG_COMPLETE: u32 = 4u32;
pub const IMGCHG_MASK: u32 = 15u32;
pub const IMGCHG_SIZE: u32 = 1u32;
pub const IMGCHG_VIEW: u32 = 2u32;
pub const IMGLOAD_COMPLETE: u32 = 16777216u32;
pub const IMGLOAD_ERROR: u32 = 8388608u32;
pub const IMGLOAD_LOADING: u32 = 2097152u32;
pub const IMGLOAD_MASK: u32 = 32505856u32;
pub const IMGLOAD_NOTLOADED: u32 = 1048576u32;
pub const IMGLOAD_STOPPED: u32 = 4194304u32;
pub const IMGTRANS_MASK: u32 = 536870912u32;
pub const IMGTRANS_OPAQUE: u32 = 536870912u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSAdminBase2W(::windows::runtime::IUnknown);
impl IMSAdminBase2W {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteChildKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pszmdname),
            ::std::mem::transmute(dwmdenumobjectindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        bmdoverwriteflag: Param4,
        bmdcopyflag: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            bmdoverwriteflag.into_param().abi(),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdnewname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            pszmdnewname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(dwmdenumdataindex),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(pdwmdnumdataentries),
            ::std::mem::transmute(pdwmddatasetnumber),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pbmdbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPaths<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pszbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdaccessrequested),
            ::std::mem::transmute(dwmdtimeout),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
        )
        .ok()
    }
    pub unsafe fn ChangePermissions(
        &self,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            ::std::mem::transmute(dwmdtimeout),
            ::std::mem::transmute(dwmdaccessrequested),
        )
        .ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetHandleInfo(
        &self,
        hmdhandle: u32,
    ) -> ::windows::runtime::Result<METADATA_HANDLE_INFO> {
        let mut result__: <METADATA_HANDLE_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            &mut result__,
        )
        .from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataSetNumber<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Restore<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(pdwmdversion),
            ::std::mem::transmute(pftmdbackuptime),
            ::std::mem::transmute(dwmdenumindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteBackup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
        )
        .ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::runtime::Result<IMSAdminBaseW> {
        let mut result__: <IMSAdminBaseW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSAdminBaseW>(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupWithPasswd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
            pszpasswd.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreWithPasswd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
            pszpasswd.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Export<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpasswd: Param0,
        pszfilename: Param1,
        pszsourcepath: Param2,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            pszpasswd.into_param().abi(),
            pszfilename.into_param().abi(),
            pszsourcepath.into_param().abi(),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Import<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpasswd: Param0,
        pszfilename: Param1,
        pszsourcepath: Param2,
        pszdestpath: Param3,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            pszpasswd.into_param().abi(),
            pszfilename.into_param().abi(),
            pszsourcepath.into_param().abi(),
            pszdestpath.into_param().abi(),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreHistory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdhistorylocation: Param0,
        dwmdmajorversion: u32,
        dwmdminorversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            pszmdhistorylocation.into_param().abi(),
            ::std::mem::transmute(dwmdmajorversion),
            ::std::mem::transmute(dwmdminorversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdhistorylocation: Param0,
        pdwmdmajorversion: *mut u32,
        pdwmdminorversion: *mut u32,
        pftmdhistorytime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            pszmdhistorylocation.into_param().abi(),
            ::std::mem::transmute(pdwmdmajorversion),
            ::std::mem::transmute(pdwmdminorversion),
            ::std::mem::transmute(pftmdhistorytime),
            ::std::mem::transmute(dwmdenumindex),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSAdminBase2W {
    type Vtable = IMSAdminBase2W_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2191053057,
        63890,
        17335,
        [142, 202, 80, 82, 216, 133, 185, 149],
    );
}
impl ::std::convert::From<IMSAdminBase2W> for ::windows::runtime::IUnknown {
    fn from(value: IMSAdminBase2W) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBase2W> for ::windows::runtime::IUnknown {
    fn from(value: &IMSAdminBase2W) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSAdminBase2W {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSAdminBase2W {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSAdminBase2W> for IMSAdminBaseW {
    fn from(value: IMSAdminBase2W) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBase2W> for IMSAdminBaseW {
    fn from(value: &IMSAdminBase2W) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBaseW> for IMSAdminBase2W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBaseW> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBaseW>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBaseW> for &IMSAdminBase2W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBaseW> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBaseW>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBase2W_abi(
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
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        bmdoverwriteflag: super::super::Foundation::BOOL,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdnewname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
        phmdnewhandle: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pmdhiinfo: *mut METADATA_HANDLE_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwsystemchangenumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pdwmddatasetnumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piadmbwinterface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpasswd: super::super::Foundation::PWSTR,
        pszfilename: super::super::Foundation::PWSTR,
        pszsourcepath: super::super::Foundation::PWSTR,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpasswd: super::super::Foundation::PWSTR,
        pszfilename: super::super::Foundation::PWSTR,
        pszsourcepath: super::super::Foundation::PWSTR,
        pszdestpath: super::super::Foundation::PWSTR,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdhistorylocation: super::super::Foundation::PWSTR,
        dwmdmajorversion: u32,
        dwmdminorversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdhistorylocation: super::super::Foundation::PWSTR,
        pdwmdmajorversion: *mut u32,
        pdwmdminorversion: *mut u32,
        pftmdhistorytime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
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
pub struct IMSAdminBase3W(::windows::runtime::IUnknown);
impl IMSAdminBase3W {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteChildKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pszmdname),
            ::std::mem::transmute(dwmdenumobjectindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        bmdoverwriteflag: Param4,
        bmdcopyflag: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            bmdoverwriteflag.into_param().abi(),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdnewname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            pszmdnewname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(dwmdenumdataindex),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(pdwmdnumdataentries),
            ::std::mem::transmute(pdwmddatasetnumber),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pbmdbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPaths<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pszbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdaccessrequested),
            ::std::mem::transmute(dwmdtimeout),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
        )
        .ok()
    }
    pub unsafe fn ChangePermissions(
        &self,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            ::std::mem::transmute(dwmdtimeout),
            ::std::mem::transmute(dwmdaccessrequested),
        )
        .ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetHandleInfo(
        &self,
        hmdhandle: u32,
    ) -> ::windows::runtime::Result<METADATA_HANDLE_INFO> {
        let mut result__: <METADATA_HANDLE_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            &mut result__,
        )
        .from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataSetNumber<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Restore<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(pdwmdversion),
            ::std::mem::transmute(pftmdbackuptime),
            ::std::mem::transmute(dwmdenumindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteBackup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
        )
        .ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::runtime::Result<IMSAdminBaseW> {
        let mut result__: <IMSAdminBaseW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSAdminBaseW>(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackupWithPasswd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
            pszpasswd.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreWithPasswd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
            pszpasswd.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Export<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpasswd: Param0,
        pszfilename: Param1,
        pszsourcepath: Param2,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            pszpasswd.into_param().abi(),
            pszfilename.into_param().abi(),
            pszsourcepath.into_param().abi(),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Import<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszpasswd: Param0,
        pszfilename: Param1,
        pszsourcepath: Param2,
        pszdestpath: Param3,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            pszpasswd.into_param().abi(),
            pszfilename.into_param().abi(),
            pszsourcepath.into_param().abi(),
            pszdestpath.into_param().abi(),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreHistory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdhistorylocation: Param0,
        dwmdmajorversion: u32,
        dwmdminorversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            pszmdhistorylocation.into_param().abi(),
            ::std::mem::transmute(dwmdmajorversion),
            ::std::mem::transmute(dwmdminorversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdhistorylocation: Param0,
        pdwmdmajorversion: *mut u32,
        pdwmdminorversion: *mut u32,
        pftmdhistorytime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            pszmdhistorylocation.into_param().abi(),
            ::std::mem::transmute(pdwmdmajorversion),
            ::std::mem::transmute(pdwmdminorversion),
            ::std::mem::transmute(pftmdhistorytime),
            ::std::mem::transmute(dwmdenumindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetChildPaths<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        cchmdbuffersize: u32,
        pszbuffer: Param3,
        pcchmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(cchmdbuffersize),
            pszbuffer.into_param().abi(),
            ::std::mem::transmute(pcchmdrequiredbuffersize),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSAdminBase3W {
    type Vtable = IMSAdminBase3W_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4128413005,
        15115,
        19542,
        [149, 99, 34, 123, 123, 230, 36, 180],
    );
}
impl ::std::convert::From<IMSAdminBase3W> for ::windows::runtime::IUnknown {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBase3W> for ::windows::runtime::IUnknown {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IMSAdminBase3W> for IMSAdminBase2W {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBase3W> for IMSAdminBase2W {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBase2W> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBase2W> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBase2W>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBase2W> for &IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBase2W> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBase2W>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IMSAdminBase3W> for IMSAdminBaseW {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBase3W> for IMSAdminBaseW {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBaseW> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBaseW> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBaseW>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMSAdminBaseW> for &IMSAdminBase3W {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMSAdminBaseW> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMSAdminBaseW>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBase3W_abi(
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
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        bmdoverwriteflag: super::super::Foundation::BOOL,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdnewname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
        phmdnewhandle: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pmdhiinfo: *mut METADATA_HANDLE_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwsystemchangenumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pdwmddatasetnumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piadmbwinterface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
        pszpasswd: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpasswd: super::super::Foundation::PWSTR,
        pszfilename: super::super::Foundation::PWSTR,
        pszsourcepath: super::super::Foundation::PWSTR,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszpasswd: super::super::Foundation::PWSTR,
        pszfilename: super::super::Foundation::PWSTR,
        pszsourcepath: super::super::Foundation::PWSTR,
        pszdestpath: super::super::Foundation::PWSTR,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdhistorylocation: super::super::Foundation::PWSTR,
        dwmdmajorversion: u32,
        dwmdminorversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdhistorylocation: super::super::Foundation::PWSTR,
        pdwmdmajorversion: *mut u32,
        pdwmdminorversion: *mut u32,
        pftmdhistorytime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        cchmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pcchmdrequiredbuffersize: *mut u32,
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
pub struct IMSAdminBaseSinkW(::windows::runtime::IUnknown);
impl IMSAdminBaseSinkW {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SinkNotify(
        &self,
        dwmdnumelements: u32,
        pcochangelist: *const MD_CHANGE_OBJECT_W,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwmdnumelements),
            ::std::mem::transmute(pcochangelist),
        )
        .ok()
    }
    pub unsafe fn ShutdownNotify(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSAdminBaseSinkW {
    type Vtable = IMSAdminBaseSinkW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2850461202,
        47117,
        4560,
        [185, 185, 0, 160, 201, 34, 231, 80],
    );
}
impl ::std::convert::From<IMSAdminBaseSinkW> for ::windows::runtime::IUnknown {
    fn from(value: IMSAdminBaseSinkW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBaseSinkW> for ::windows::runtime::IUnknown {
    fn from(value: &IMSAdminBaseSinkW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSAdminBaseSinkW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSAdminBaseSinkW {
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
pub struct IMSAdminBaseSinkW_abi(
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
        dwmdnumelements: u32,
        pcochangelist: *const MD_CHANGE_OBJECT_W,
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
pub struct IMSAdminBaseW(::windows::runtime::IUnknown);
impl IMSAdminBaseW {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteChildKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumKeys<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pszmdname),
            ::std::mem::transmute(dwmdenumobjectindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        bmdoverwriteflag: Param4,
        bmdcopyflag: Param5,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            bmdoverwriteflag.into_param().abi(),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pszmdnewname: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            pszmdnewname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pmdrmddata),
            ::std::mem::transmute(dwmdenumdataindex),
            ::std::mem::transmute(pdwmdrequireddatalen),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(pdwmdnumdataentries),
            ::std::mem::transmute(pdwmddatasetnumber),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pbmdbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteAllData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdsourcehandle: u32,
        pszmdsourcepath: Param1,
        hmddesthandle: u32,
        pszmddestpath: Param3,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdsourcehandle),
            pszmdsourcepath.into_param().abi(),
            ::std::mem::transmute(hmddesthandle),
            pszmddestpath.into_param().abi(),
            ::std::mem::transmute(dwmdattributes),
            ::std::mem::transmute(dwmdusertype),
            ::std::mem::transmute(dwmddatatype),
            bmdcopyflag.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPaths<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdidentifier),
            ::std::mem::transmute(dwmddatatype),
            ::std::mem::transmute(dwmdbuffersize),
            ::std::mem::transmute(pszbuffer),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenKey<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(dwmdaccessrequested),
            ::std::mem::transmute(dwmdtimeout),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
        )
        .ok()
    }
    pub unsafe fn ChangePermissions(
        &self,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            ::std::mem::transmute(dwmdtimeout),
            ::std::mem::transmute(dwmdaccessrequested),
        )
        .ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetHandleInfo(
        &self,
        hmdhandle: u32,
    ) -> ::windows::runtime::Result<METADATA_HANDLE_INFO> {
        let mut result__: <METADATA_HANDLE_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            &mut result__,
        )
        .from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataSetNumber<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hmdhandle: u32,
        pszmdpath: Param1,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hmdhandle),
            pszmdpath.into_param().abi(),
            ::std::mem::transmute(pftmdlastchangetime),
            blocaltime.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Restore<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
            ::std::mem::transmute(dwmdflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(pdwmdversion),
            ::std::mem::transmute(pftmdbackuptime),
            ::std::mem::transmute(dwmdenumindex),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteBackup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmdbackuplocation: Param0,
        dwmdversion: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            pszmdbackuplocation.into_param().abi(),
            ::std::mem::transmute(dwmdversion),
        )
        .ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::runtime::Result<IMSAdminBaseW> {
        let mut result__: <IMSAdminBaseW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMSAdminBaseW>(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSAdminBaseW {
    type Vtable = IMSAdminBaseW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1890915376,
        46794,
        4560,
        [185, 185, 0, 160, 201, 34, 231, 80],
    );
}
impl ::std::convert::From<IMSAdminBaseW> for ::windows::runtime::IUnknown {
    fn from(value: IMSAdminBaseW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSAdminBaseW> for ::windows::runtime::IUnknown {
    fn from(value: &IMSAdminBaseW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSAdminBaseW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSAdminBaseW {
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
pub struct IMSAdminBaseW_abi(
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
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdname: super::super::Foundation::PWSTR,
        dwmdenumobjectindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        bmdoverwriteflag: super::super::Foundation::BOOL,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pszmdnewname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pmdrmddata: *mut METADATA_RECORD,
        dwmdenumdataindex: u32,
        pdwmdrequireddatalen: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        pdwmdnumdataentries: *mut u32,
        pdwmddatasetnumber: *mut u32,
        dwmdbuffersize: u32,
        pbmdbuffer: *mut u8,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdusertype: u32,
        dwmddatatype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdsourcehandle: u32,
        pszmdsourcepath: super::super::Foundation::PWSTR,
        hmddesthandle: u32,
        pszmddestpath: super::super::Foundation::PWSTR,
        dwmdattributes: u32,
        dwmdusertype: u32,
        dwmddatatype: u32,
        bmdcopyflag: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdidentifier: u32,
        dwmddatatype: u32,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        dwmdaccessrequested: u32,
        dwmdtimeout: u32,
        phmdnewhandle: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        dwmdtimeout: u32,
        dwmdaccessrequested: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pmdhiinfo: *mut METADATA_HANDLE_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwsystemchangenumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pdwmddatasetnumber: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *const super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hmdhandle: u32,
        pszmdpath: super::super::Foundation::PWSTR,
        pftmdlastchangetime: *mut super::super::Foundation::FILETIME,
        blocaltime: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
        dwmdflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        pdwmdversion: *mut u32,
        pftmdbackuptime: *mut super::super::Foundation::FILETIME,
        dwmdenumindex: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmdbackuplocation: super::super::Foundation::PWSTR,
        dwmdversion: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piadmbwinterface: *mut ::windows::runtime::RawPtr,
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
pub struct IMSImpExpHelpW(::windows::runtime::IUnknown);
impl IMSImpExpHelpW {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumeratePathsInFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszfilename: Param0,
        pszkeytype: Param1,
        dwmdbuffersize: u32,
        pszbuffer: Param3,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfilename.into_param().abi(),
            pszkeytype.into_param().abi(),
            ::std::mem::transmute(dwmdbuffersize),
            pszbuffer.into_param().abi(),
            ::std::mem::transmute(pdwmdrequiredbuffersize),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSImpExpHelpW {
    type Vtable = IMSImpExpHelpW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        704604159,
        32848,
        18447,
        [159, 48, 204, 65, 99, 95, 47, 157],
    );
}
impl ::std::convert::From<IMSImpExpHelpW> for ::windows::runtime::IUnknown {
    fn from(value: IMSImpExpHelpW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSImpExpHelpW> for ::windows::runtime::IUnknown {
    fn from(value: &IMSImpExpHelpW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSImpExpHelpW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSImpExpHelpW {
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
pub struct IMSImpExpHelpW_abi(
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
        pszfilename: super::super::Foundation::PWSTR,
        pszkeytype: super::super::Foundation::PWSTR,
        dwmdbuffersize: u32,
        pszbuffer: super::super::Foundation::PWSTR,
        pdwmdrequiredbuffersize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const LIBID_ASPTypeLibrary: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3648679328,
    43100,
    4559,
    [131, 174, 0, 160, 201, 12, 43, 216],
);
pub const LIBID_IISRSTALib: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3908797972,
    22671,
    4562,
    [157, 97, 0, 192, 79, 121, 197, 254],
);
pub const LIBID_WAMREGLib: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    696396456,
    62210,
    4560,
    [153, 83, 0, 192, 79, 217, 25, 193],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGGING_PARAMETERS {
    pub pszSessionId: super::super::Foundation::PWSTR,
    pub pszSiteName: super::super::Foundation::PWSTR,
    pub pszUserName: super::super::Foundation::PWSTR,
    pub pszHostName: super::super::Foundation::PWSTR,
    pub pszRemoteIpAddress: super::super::Foundation::PWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: super::super::Foundation::PWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: super::super::Foundation::PWSTR,
    pub pszCommandParameters: super::super::Foundation::PWSTR,
    pub pszFullPath: super::super::Foundation::PWSTR,
    pub dwElapsedMilliseconds: u32,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: ::windows::runtime::HRESULT,
    pub pszInformation: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl LOGGING_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LOGGING_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LOGGING_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("dwElapsedMilliseconds", &self.dwElapsedMilliseconds)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("pszInformation", &self.pszInformation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LOGGING_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.dwElapsedMilliseconds == other.dwElapsedMilliseconds
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.pszInformation == other.pszInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LOGGING_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LOGGING_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MB_DONT_IMPERSONATE: u32 = 9033u32;
pub const MD_ACCESS_EXECUTE: u32 = 4u32;
pub const MD_ACCESS_MAP_CERT: u32 = 128u32;
pub const MD_ACCESS_MASK: u32 = 65535u32;
pub const MD_ACCESS_NEGO_CERT: u32 = 32u32;
pub const MD_ACCESS_NO_PHYSICAL_DIR: u32 = 32768u32;
pub const MD_ACCESS_NO_REMOTE_EXECUTE: u32 = 8192u32;
pub const MD_ACCESS_NO_REMOTE_READ: u32 = 4096u32;
pub const MD_ACCESS_NO_REMOTE_SCRIPT: u32 = 16384u32;
pub const MD_ACCESS_NO_REMOTE_WRITE: u32 = 1024u32;
pub const MD_ACCESS_PERM: u32 = 6016u32;
pub const MD_ACCESS_READ: u32 = 1u32;
pub const MD_ACCESS_REQUIRE_CERT: u32 = 64u32;
pub const MD_ACCESS_SCRIPT: u32 = 512u32;
pub const MD_ACCESS_SOURCE: u32 = 16u32;
pub const MD_ACCESS_SSL: u32 = 8u32;
pub const MD_ACCESS_SSL128: u32 = 256u32;
pub const MD_ACCESS_WRITE: u32 = 2u32;
pub const MD_ACR_ENUM_KEYS: u32 = 8u32;
pub const MD_ACR_READ: u32 = 1u32;
pub const MD_ACR_RESTRICTED_WRITE: u32 = 32u32;
pub const MD_ACR_UNSECURE_PROPS_READ: u32 = 128u32;
pub const MD_ACR_WRITE: u32 = 2u32;
pub const MD_ACR_WRITE_DAC: u32 = 262144u32;
pub const MD_ADMIN_ACL: u32 = 6027u32;
pub const MD_ADMIN_INSTANCE: u32 = 2115u32;
pub const MD_ADV_CACHE_TTL: u32 = 2064u32;
pub const MD_ADV_NOTIFY_PWD_EXP_IN_DAYS: u32 = 2063u32;
pub const MD_AD_CONNECTIONS_PASSWORD: u32 = 5015u32;
pub const MD_AD_CONNECTIONS_USERNAME: u32 = 5014u32;
pub const MD_ALLOW_ANONYMOUS: u32 = 5005u32;
pub const MD_ALLOW_KEEPALIVES: u32 = 6038u32;
pub const MD_ALLOW_PATH_INFO_FOR_SCRIPT_MAPPINGS: u32 = 2095u32;
pub const MD_ALLOW_REPLACE_ON_RENAME: u32 = 5009u32;
pub const MD_ANONYMOUS_ONLY: u32 = 5006u32;
pub const MD_ANONYMOUS_PWD: u32 = 6021u32;
pub const MD_ANONYMOUS_USER_NAME: u32 = 6020u32;
pub const MD_ANONYMOUS_USE_SUBAUTH: u32 = 6022u32;
pub const MD_APPPOOL_32_BIT_APP_ON_WIN64: u32 = 9040u32;
pub const MD_APPPOOL_ALLOW_TRANSIENT_REGISTRATION: u32 = 9202u32;
pub const MD_APPPOOL_APPPOOL_ID: u32 = 9201u32;
pub const MD_APPPOOL_AUTO_SHUTDOWN_EXE: u32 = 9035u32;
pub const MD_APPPOOL_AUTO_SHUTDOWN_PARAMS: u32 = 9036u32;
pub const MD_APPPOOL_AUTO_START: u32 = 9028u32;
pub const MD_APPPOOL_COMMAND: u32 = 9026u32;
pub const MD_APPPOOL_COMMAND_START: u32 = 1u32;
pub const MD_APPPOOL_COMMAND_STOP: u32 = 2u32;
pub const MD_APPPOOL_DISALLOW_OVERLAPPING_ROTATION: u32 = 9015u32;
pub const MD_APPPOOL_DISALLOW_ROTATION_ON_CONFIG_CHANGE: u32 = 9018u32;
pub const MD_APPPOOL_IDENTITY_TYPE: u32 = 9021u32;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSERVICE: u32 = 1u32;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSYSTEM: u32 = 0u32;
pub const MD_APPPOOL_IDENTITY_TYPE_NETWORKSERVICE: u32 = 2u32;
pub const MD_APPPOOL_IDENTITY_TYPE_SPECIFICUSER: u32 = 3u32;
pub const MD_APPPOOL_IDLE_TIMEOUT: u32 = 9005u32;
pub const MD_APPPOOL_MANAGED_PIPELINE_MODE: u32 = 9041u32;
pub const MD_APPPOOL_MANAGED_RUNTIME_VERSION: u32 = 9039u32;
pub const MD_APPPOOL_MAX_PROCESS_COUNT: u32 = 9003u32;
pub const MD_APPPOOL_ORPHAN_ACTION_EXE: u32 = 9031u32;
pub const MD_APPPOOL_ORPHAN_ACTION_PARAMS: u32 = 9032u32;
pub const MD_APPPOOL_ORPHAN_PROCESSES_FOR_DEBUGGING: u32 = 9009u32;
pub const MD_APPPOOL_PERIODIC_RESTART_CONNECTIONS: u32 = 9104u32;
pub const MD_APPPOOL_PERIODIC_RESTART_MEMORY: u32 = 9024u32;
pub const MD_APPPOOL_PERIODIC_RESTART_PRIVATE_MEMORY: u32 = 9038u32;
pub const MD_APPPOOL_PERIODIC_RESTART_REQUEST_COUNT: u32 = 9002u32;
pub const MD_APPPOOL_PERIODIC_RESTART_SCHEDULE: u32 = 9020u32;
pub const MD_APPPOOL_PERIODIC_RESTART_TIME: u32 = 9001u32;
pub const MD_APPPOOL_PINGING_ENABLED: u32 = 9004u32;
pub const MD_APPPOOL_PING_INTERVAL: u32 = 9013u32;
pub const MD_APPPOOL_PING_RESPONSE_TIMELIMIT: u32 = 9014u32;
pub const MD_APPPOOL_RAPID_FAIL_PROTECTION_ENABLED: u32 = 9006u32;
pub const MD_APPPOOL_SHUTDOWN_TIMELIMIT: u32 = 9012u32;
pub const MD_APPPOOL_SMP_AFFINITIZED: u32 = 9007u32;
pub const MD_APPPOOL_SMP_AFFINITIZED_PROCESSOR_MASK: u32 = 9008u32;
pub const MD_APPPOOL_STARTUP_TIMELIMIT: u32 = 9011u32;
pub const MD_APPPOOL_STATE: u32 = 9027u32;
pub const MD_APPPOOL_STATE_STARTED: u32 = 2u32;
pub const MD_APPPOOL_STATE_STARTING: u32 = 1u32;
pub const MD_APPPOOL_STATE_STOPPED: u32 = 4u32;
pub const MD_APPPOOL_STATE_STOPPING: u32 = 3u32;
pub const MD_APPPOOL_UL_APPPOOL_QUEUE_LENGTH: u32 = 9017u32;
pub const MD_APP_ALLOW_TRANSIENT_REGISTRATION: u32 = 9102u32;
pub const MD_APP_APPPOOL_ID: u32 = 9101u32;
pub const MD_APP_AUTO_START: u32 = 9103u32;
pub const MD_APP_DEPENDENCIES: u32 = 2167u32;
pub const MD_APP_FRIENDLY_NAME: u32 = 2102u32;
pub const MD_APP_ISOLATED: u32 = 2104u32;
pub const MD_APP_OOP_RECOVER_LIMIT: u32 = 2110u32;
pub const MD_APP_PACKAGE_ID: u32 = 2106u32;
pub const MD_APP_PACKAGE_NAME: u32 = 2107u32;
pub const MD_APP_PERIODIC_RESTART_REQUESTS: u32 = 2112u32;
pub const MD_APP_PERIODIC_RESTART_SCHEDULE: u32 = 2113u32;
pub const MD_APP_PERIODIC_RESTART_TIME: u32 = 2111u32;
pub const MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL: u32 = 9042u32;
pub const MD_APP_POOL_LOG_EVENT_ON_RECYCLE: u32 = 9037u32;
pub const MD_APP_POOL_PROCESSMODEL_IDLE_TIMEOUT: u32 = 1u32;
pub const MD_APP_POOL_RECYCLE_CONFIG_CHANGE: u32 = 64u32;
pub const MD_APP_POOL_RECYCLE_ISAPI_UNHEALTHY: u32 = 16u32;
pub const MD_APP_POOL_RECYCLE_MEMORY: u32 = 8u32;
pub const MD_APP_POOL_RECYCLE_ON_DEMAND: u32 = 32u32;
pub const MD_APP_POOL_RECYCLE_PRIVATE_MEMORY: u32 = 128u32;
pub const MD_APP_POOL_RECYCLE_REQUESTS: u32 = 2u32;
pub const MD_APP_POOL_RECYCLE_SCHEDULE: u32 = 4u32;
pub const MD_APP_POOL_RECYCLE_TIME: u32 = 1u32;
pub const MD_APP_ROOT: u32 = 2103u32;
pub const MD_APP_SHUTDOWN_TIME_LIMIT: u32 = 2114u32;
pub const MD_APP_TRACE_URL_LIST: u32 = 2118u32;
pub const MD_APP_WAM_CLSID: u32 = 2105u32;
pub const MD_ASP_ALLOWOUTOFPROCCMPNTS: u32 = 7014u32;
pub const MD_ASP_ALLOWOUTOFPROCCOMPONENTS: u32 = 7014u32;
pub const MD_ASP_ALLOWSESSIONSTATE: u32 = 7011u32;
pub const MD_ASP_BUFFERINGON: u32 = 7000u32;
pub const MD_ASP_BUFFER_LIMIT: u32 = 7052u32;
pub const MD_ASP_CALCLINENUMBER: u32 = 7050u32;
pub const MD_ASP_CODEPAGE: u32 = 7016u32;
pub const MD_ASP_DISKTEMPLATECACHEDIRECTORY: u32 = 7036u32;
pub const MD_ASP_ENABLEAPPLICATIONRESTART: u32 = 7027u32;
pub const MD_ASP_ENABLEASPHTMLFALLBACK: u32 = 7021u32;
pub const MD_ASP_ENABLECHUNKEDENCODING: u32 = 7022u32;
pub const MD_ASP_ENABLECLIENTDEBUG: u32 = 7019u32;
pub const MD_ASP_ENABLEPARENTPATHS: u32 = 7008u32;
pub const MD_ASP_ENABLESERVERDEBUG: u32 = 7018u32;
pub const MD_ASP_ENABLETYPELIBCACHE: u32 = 7023u32;
pub const MD_ASP_ERRORSTONTLOG: u32 = 7024u32;
pub const MD_ASP_EXCEPTIONCATCHENABLE: u32 = 7015u32;
pub const MD_ASP_EXECUTEINMTA: u32 = 7041u32;
pub const MD_ASP_ID_LAST: u32 = 7053u32;
pub const MD_ASP_KEEPSESSIONIDSECURE: u32 = 7043u32;
pub const MD_ASP_LCID: u32 = 7042u32;
pub const MD_ASP_LOGERRORREQUESTS: u32 = 7001u32;
pub const MD_ASP_MAXDISKTEMPLATECACHEFILES: u32 = 7040u32;
pub const MD_ASP_MAXREQUESTENTITY: u32 = 7053u32;
pub const MD_ASP_MAX_REQUEST_ENTITY_ALLOWED: u32 = 7053u32;
pub const MD_ASP_MEMFREEFACTOR: u32 = 7009u32;
pub const MD_ASP_MINUSEDBLOCKS: u32 = 7010u32;
pub const MD_ASP_PROCESSORTHREADMAX: u32 = 7025u32;
pub const MD_ASP_QUEUECONNECTIONTESTTIME: u32 = 7028u32;
pub const MD_ASP_QUEUETIMEOUT: u32 = 7013u32;
pub const MD_ASP_REQEUSTQUEUEMAX: u32 = 7026u32;
pub const MD_ASP_RUN_ONEND_ANON: u32 = 7051u32;
pub const MD_ASP_SCRIPTENGINECACHEMAX: u32 = 7005u32;
pub const MD_ASP_SCRIPTERRORMESSAGE: u32 = 7003u32;
pub const MD_ASP_SCRIPTERRORSSENTTOBROWSER: u32 = 7002u32;
pub const MD_ASP_SCRIPTFILECACHESIZE: u32 = 7004u32;
pub const MD_ASP_SCRIPTLANGUAGE: u32 = 7012u32;
pub const MD_ASP_SCRIPTLANGUAGELIST: u32 = 7017u32;
pub const MD_ASP_SCRIPTTIMEOUT: u32 = 7006u32;
pub const MD_ASP_SERVICE_ENABLE_SXS: u32 = 2u32;
pub const MD_ASP_SERVICE_ENABLE_TRACKER: u32 = 1u32;
pub const MD_ASP_SERVICE_FLAGS: u32 = 7044u32;
pub const MD_ASP_SERVICE_FLAG_FUSION: u32 = 7046u32;
pub const MD_ASP_SERVICE_FLAG_PARTITIONS: u32 = 7047u32;
pub const MD_ASP_SERVICE_FLAG_TRACKER: u32 = 7045u32;
pub const MD_ASP_SERVICE_PARTITION_ID: u32 = 7048u32;
pub const MD_ASP_SERVICE_SXS_NAME: u32 = 7049u32;
pub const MD_ASP_SERVICE_USE_PARTITION: u32 = 4u32;
pub const MD_ASP_SESSIONMAX: u32 = 7029u32;
pub const MD_ASP_SESSIONTIMEOUT: u32 = 7007u32;
pub const MD_ASP_THREADGATEENABLED: u32 = 7030u32;
pub const MD_ASP_THREADGATELOADHIGH: u32 = 7035u32;
pub const MD_ASP_THREADGATELOADLOW: u32 = 7034u32;
pub const MD_ASP_THREADGATESLEEPDELAY: u32 = 7032u32;
pub const MD_ASP_THREADGATESLEEPMAX: u32 = 7033u32;
pub const MD_ASP_THREADGATETIMESLICE: u32 = 7031u32;
pub const MD_ASP_TRACKTHREADINGMODEL: u32 = 7020u32;
pub const MD_AUTHORIZATION: u32 = 6000u32;
pub const MD_AUTHORIZATION_PERSISTENCE: u32 = 6031u32;
pub const MD_AUTH_ADVNOTIFY_DISABLE: u32 = 4u32;
pub const MD_AUTH_ANONYMOUS: u32 = 1u32;
pub const MD_AUTH_BASIC: u32 = 2u32;
pub const MD_AUTH_CHANGE_DISABLE: u32 = 2u32;
pub const MD_AUTH_CHANGE_FLAGS: u32 = 2068u32;
pub const MD_AUTH_CHANGE_UNSECURE: u32 = 1u32;
pub const MD_AUTH_CHANGE_URL: u32 = 2060u32;
pub const MD_AUTH_EXPIRED_UNSECUREURL: u32 = 2067u32;
pub const MD_AUTH_EXPIRED_URL: u32 = 2061u32;
pub const MD_AUTH_MD5: u32 = 16u32;
pub const MD_AUTH_NT: u32 = 4u32;
pub const MD_AUTH_PASSPORT: u32 = 64u32;
pub const MD_AUTH_SINGLEREQUEST: u32 = 64u32;
pub const MD_AUTH_SINGLEREQUESTALWAYSIFPROXY: u32 = 256u32;
pub const MD_AUTH_SINGLEREQUESTIFPROXY: u32 = 128u32;
pub const MD_BACKUP_FORCE_BACKUP: u32 = 4u32;
pub const MD_BACKUP_HIGHEST_VERSION: u32 = 4294967294u32;
pub const MD_BACKUP_MAX_LEN: u32 = 100u32;
pub const MD_BACKUP_MAX_VERSION: u32 = 9999u32;
pub const MD_BACKUP_NEXT_VERSION: u32 = 4294967295u32;
pub const MD_BACKUP_OVERWRITE: u32 = 1u32;
pub const MD_BACKUP_SAVE_FIRST: u32 = 2u32;
pub const MD_BANNER_MESSAGE: u32 = 5011u32;
pub const MD_BINDINGS: u32 = 2022u32;
pub const MD_CACHE_EXTENSIONS: u32 = 6034u32;
pub const MD_CAL_AUTH_RESERVE_TIMEOUT: u32 = 2131u32;
pub const MD_CAL_SSL_RESERVE_TIMEOUT: u32 = 2132u32;
pub const MD_CAL_VC_PER_CONNECT: u32 = 2130u32;
pub const MD_CAL_W3_ERROR: u32 = 2133u32;
pub const MD_CC_MAX_AGE: u32 = 6042u32;
pub const MD_CC_NO_CACHE: u32 = 6041u32;
pub const MD_CC_OTHER: u32 = 6043u32;
pub const MD_CENTRAL_W3C_LOGGING_ENABLED: u32 = 2119u32;
pub const MD_CERT_CACHE_RETRIEVAL_ONLY: u32 = 2u32;
pub const MD_CERT_CHECK_REVOCATION_FRESHNESS_TIME: u32 = 4u32;
pub const MD_CERT_NO_REVOC_CHECK: u32 = 1u32;
pub const MD_CERT_NO_USAGE_CHECK: u32 = 65536u32;
pub const MD_CGI_RESTRICTION_LIST: u32 = 2164u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MD_CHANGE_OBJECT_W {
    pub pszMDPath: super::super::Foundation::PWSTR,
    pub dwMDChangeType: u32,
    pub dwMDNumDataIDs: u32,
    pub pdwMDDataIDs: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MD_CHANGE_OBJECT_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MD_CHANGE_OBJECT_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MD_CHANGE_OBJECT_W")
            .field("pszMDPath", &self.pszMDPath)
            .field("dwMDChangeType", &self.dwMDChangeType)
            .field("dwMDNumDataIDs", &self.dwMDNumDataIDs)
            .field("pdwMDDataIDs", &self.pdwMDDataIDs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MD_CHANGE_OBJECT_W {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDPath == other.pszMDPath
            && self.dwMDChangeType == other.dwMDChangeType
            && self.dwMDNumDataIDs == other.dwMDNumDataIDs
            && self.pdwMDDataIDs == other.pdwMDDataIDs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MD_CHANGE_OBJECT_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MD_CHANGE_OBJECT_W {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MD_CHANGE_TYPE_ADD_OBJECT: u32 = 2u32;
pub const MD_CHANGE_TYPE_DELETE_DATA: u32 = 8u32;
pub const MD_CHANGE_TYPE_DELETE_OBJECT: u32 = 1u32;
pub const MD_CHANGE_TYPE_RENAME_OBJECT: u32 = 16u32;
pub const MD_CHANGE_TYPE_RESTORE: u32 = 32u32;
pub const MD_CHANGE_TYPE_SET_DATA: u32 = 4u32;
pub const MD_COMMENTS: u32 = 9990u32;
pub const MD_CONNECTION_TIMEOUT: u32 = 1013u32;
pub const MD_CPU_ACTION: u32 = 9022u32;
pub const MD_CPU_APP_ENABLED: u32 = 2141u32;
pub const MD_CPU_CGI_ENABLED: u32 = 2140u32;
pub const MD_CPU_CGI_LIMIT: u32 = 2148u32;
pub const MD_CPU_DISABLE_ALL_LOGGING: u32 = 0u32;
pub const MD_CPU_ENABLE_ACTIVE_PROCS: u32 = 64u32;
pub const MD_CPU_ENABLE_ALL_PROC_LOGGING: u32 = 1u32;
pub const MD_CPU_ENABLE_APP_LOGGING: u32 = 4u32;
pub const MD_CPU_ENABLE_CGI_LOGGING: u32 = 2u32;
pub const MD_CPU_ENABLE_EVENT: u32 = 1u32;
pub const MD_CPU_ENABLE_KERNEL_TIME: u32 = 8u32;
pub const MD_CPU_ENABLE_LOGGING: u32 = 2147483648u32;
pub const MD_CPU_ENABLE_PAGE_FAULTS: u32 = 16u32;
pub const MD_CPU_ENABLE_PROC_TYPE: u32 = 2u32;
pub const MD_CPU_ENABLE_TERMINATED_PROCS: u32 = 128u32;
pub const MD_CPU_ENABLE_TOTAL_PROCS: u32 = 32u32;
pub const MD_CPU_ENABLE_USER_TIME: u32 = 4u32;
pub const MD_CPU_KILL_W3WP: u32 = 1u32;
pub const MD_CPU_LIMIT: u32 = 9023u32;
pub const MD_CPU_LIMITS_ENABLED: u32 = 2143u32;
pub const MD_CPU_LIMIT_LOGEVENT: u32 = 2149u32;
pub const MD_CPU_LIMIT_PAUSE: u32 = 2152u32;
pub const MD_CPU_LIMIT_PRIORITY: u32 = 2150u32;
pub const MD_CPU_LIMIT_PROCSTOP: u32 = 2151u32;
pub const MD_CPU_LOGGING_INTERVAL: u32 = 2145u32;
pub const MD_CPU_LOGGING_MASK: u32 = 4507u32;
pub const MD_CPU_LOGGING_OPTIONS: u32 = 2146u32;
pub const MD_CPU_NO_ACTION: u32 = 0u32;
pub const MD_CPU_RESET_INTERVAL: u32 = 2144u32;
pub const MD_CPU_THROTTLE: u32 = 3u32;
pub const MD_CPU_TRACE: u32 = 2u32;
pub const MD_CREATE_PROCESS_AS_USER: u32 = 6035u32;
pub const MD_CREATE_PROC_NEW_CONSOLE: u32 = 6036u32;
pub const MD_CUSTOM_DEPLOYMENT_DATA: u32 = 6055u32;
pub const MD_CUSTOM_ERROR: u32 = 6008u32;
pub const MD_CUSTOM_ERROR_DESC: u32 = 2120u32;
pub const MD_DEFAULT_LOAD_FILE: u32 = 6006u32;
pub const MD_DEFAULT_LOGON_DOMAIN: u32 = 6012u32;
pub const MD_DEMAND_START_THRESHOLD: u32 = 9207u32;
pub const MD_DIRBROW_ENABLED: u32 = 2147483648u32;
pub const MD_DIRBROW_LOADDEFAULT: u32 = 1073741824u32;
pub const MD_DIRBROW_LONG_DATE: u32 = 32u32;
pub const MD_DIRBROW_SHOW_DATE: u32 = 2u32;
pub const MD_DIRBROW_SHOW_EXTENSION: u32 = 16u32;
pub const MD_DIRBROW_SHOW_SIZE: u32 = 8u32;
pub const MD_DIRBROW_SHOW_TIME: u32 = 4u32;
pub const MD_DIRECTORY_BROWSING: u32 = 6005u32;
pub const MD_DISABLE_SOCKET_POOLING: u32 = 1029u32;
pub const MD_DONT_LOG: u32 = 6023u32;
pub const MD_DOWNLEVEL_ADMIN_INSTANCE: u32 = 1021u32;
pub const MD_DO_REVERSE_DNS: u32 = 6029u32;
pub const MD_ENABLEDPROTOCOLS: u32 = 2023u32;
pub const MD_ENABLE_URL_AUTHORIZATION: u32 = 6048u32;
pub const MD_ERROR_CANNOT_REMOVE_SECURE_ATTRIBUTE: i32 = -2146646008i32;
pub const MD_ERROR_DATA_NOT_FOUND: i32 = -2146646015i32;
pub const MD_ERROR_IISAO_INVALID_SCHEMA: i32 = -2146646000i32;
pub const MD_ERROR_INVALID_VERSION: i32 = -2146646014i32;
pub const MD_ERROR_NOT_INITIALIZED: i32 = -2146646016i32;
pub const MD_ERROR_NO_SESSION_KEY: i32 = -2146645987i32;
pub const MD_ERROR_READ_METABASE_FILE: i32 = -2146645991i32;
pub const MD_ERROR_SECURE_CHANNEL_FAILURE: i32 = -2146646010i32;
pub const MD_ERROR_SUB400_INVALID_CONTENT_LENGTH: u32 = 7u32;
pub const MD_ERROR_SUB400_INVALID_DEPTH: u32 = 2u32;
pub const MD_ERROR_SUB400_INVALID_DESTINATION: u32 = 1u32;
pub const MD_ERROR_SUB400_INVALID_IF: u32 = 3u32;
pub const MD_ERROR_SUB400_INVALID_LOCK_TOKEN: u32 = 9u32;
pub const MD_ERROR_SUB400_INVALID_OVERWRITE: u32 = 4u32;
pub const MD_ERROR_SUB400_INVALID_REQUEST_BODY: u32 = 6u32;
pub const MD_ERROR_SUB400_INVALID_TIMEOUT: u32 = 8u32;
pub const MD_ERROR_SUB400_INVALID_TRANSLATE: u32 = 5u32;
pub const MD_ERROR_SUB400_INVALID_WEBSOCKET_REQUEST: u32 = 11u32;
pub const MD_ERROR_SUB400_INVALID_XFF_HEADER: u32 = 10u32;
pub const MD_ERROR_SUB401_APPLICATION: u32 = 5u32;
pub const MD_ERROR_SUB401_FILTER: u32 = 4u32;
pub const MD_ERROR_SUB401_LOGON: u32 = 1u32;
pub const MD_ERROR_SUB401_LOGON_ACL: u32 = 3u32;
pub const MD_ERROR_SUB401_LOGON_CONFIG: u32 = 2u32;
pub const MD_ERROR_SUB401_URLAUTH_POLICY: u32 = 7u32;
pub const MD_ERROR_SUB403_ADDR_REJECT: u32 = 6u32;
pub const MD_ERROR_SUB403_APPPOOL_DENIED: u32 = 18u32;
pub const MD_ERROR_SUB403_CAL_EXCEEDED: u32 = 15u32;
pub const MD_ERROR_SUB403_CERT_BAD: u32 = 16u32;
pub const MD_ERROR_SUB403_CERT_REQUIRED: u32 = 7u32;
pub const MD_ERROR_SUB403_CERT_REVOKED: u32 = 13u32;
pub const MD_ERROR_SUB403_CERT_TIME_INVALID: u32 = 17u32;
pub const MD_ERROR_SUB403_DIR_LIST_DENIED: u32 = 14u32;
pub const MD_ERROR_SUB403_EXECUTE_ACCESS_DENIED: u32 = 1u32;
pub const MD_ERROR_SUB403_INFINITE_DEPTH_DENIED: u32 = 22u32;
pub const MD_ERROR_SUB403_INSUFFICIENT_PRIVILEGE_FOR_CGI: u32 = 19u32;
pub const MD_ERROR_SUB403_INVALID_CNFG: u32 = 10u32;
pub const MD_ERROR_SUB403_LOCK_TOKEN_REQUIRED: u32 = 23u32;
pub const MD_ERROR_SUB403_MAPPER_DENY_ACCESS: u32 = 12u32;
pub const MD_ERROR_SUB403_PASSPORT_LOGIN_FAILURE: u32 = 20u32;
pub const MD_ERROR_SUB403_PWD_CHANGE: u32 = 11u32;
pub const MD_ERROR_SUB403_READ_ACCESS_DENIED: u32 = 2u32;
pub const MD_ERROR_SUB403_SITE_ACCESS_DENIED: u32 = 8u32;
pub const MD_ERROR_SUB403_SOURCE_ACCESS_DENIED: u32 = 21u32;
pub const MD_ERROR_SUB403_SSL128_REQUIRED: u32 = 5u32;
pub const MD_ERROR_SUB403_SSL_REQUIRED: u32 = 4u32;
pub const MD_ERROR_SUB403_TOO_MANY_USERS: u32 = 9u32;
pub const MD_ERROR_SUB403_VALIDATION_FAILURE: u32 = 24u32;
pub const MD_ERROR_SUB403_WRITE_ACCESS_DENIED: u32 = 3u32;
pub const MD_ERROR_SUB404_DENIED_BY_FILTERING_RULE: u32 = 19u32;
pub const MD_ERROR_SUB404_DENIED_BY_MIMEMAP: u32 = 3u32;
pub const MD_ERROR_SUB404_DENIED_BY_POLICY: u32 = 2u32;
pub const MD_ERROR_SUB404_FILE_ATTRIBUTE_HIDDEN: u32 = 9u32;
pub const MD_ERROR_SUB404_FILE_EXTENSION_DENIED: u32 = 7u32;
pub const MD_ERROR_SUB404_HIDDEN_SEGMENT: u32 = 8u32;
pub const MD_ERROR_SUB404_NO_HANDLER: u32 = 4u32;
pub const MD_ERROR_SUB404_PRECONDITIONED_HANDLER: u32 = 17u32;
pub const MD_ERROR_SUB404_QUERY_STRING_SEQUENCE_DENIED: u32 = 18u32;
pub const MD_ERROR_SUB404_QUERY_STRING_TOO_LONG: u32 = 15u32;
pub const MD_ERROR_SUB404_SITE_NOT_FOUND: u32 = 1u32;
pub const MD_ERROR_SUB404_STATICFILE_DAV: u32 = 16u32;
pub const MD_ERROR_SUB404_TOO_MANY_URL_SEGMENTS: u32 = 20u32;
pub const MD_ERROR_SUB404_URL_DOUBLE_ESCAPED: u32 = 11u32;
pub const MD_ERROR_SUB404_URL_HAS_HIGH_BIT_CHARS: u32 = 12u32;
pub const MD_ERROR_SUB404_URL_SEQUENCE_DENIED: u32 = 5u32;
pub const MD_ERROR_SUB404_URL_TOO_LONG: u32 = 14u32;
pub const MD_ERROR_SUB404_VERB_DENIED: u32 = 6u32;
pub const MD_ERROR_SUB413_CONTENT_LENGTH_TOO_LARGE: u32 = 1u32;
pub const MD_ERROR_SUB423_LOCK_TOKEN_SUBMITTED: u32 = 1u32;
pub const MD_ERROR_SUB423_NO_CONFLICTING_LOCK: u32 = 2u32;
pub const MD_ERROR_SUB500_ASPNET_HANDLERS: u32 = 23u32;
pub const MD_ERROR_SUB500_ASPNET_IMPERSONATION: u32 = 24u32;
pub const MD_ERROR_SUB500_ASPNET_MODULES: u32 = 22u32;
pub const MD_ERROR_SUB500_BAD_METADATA: u32 = 19u32;
pub const MD_ERROR_SUB500_HANDLERS_MODULE: u32 = 21u32;
pub const MD_ERROR_SUB500_UNC_ACCESS: u32 = 16u32;
pub const MD_ERROR_SUB500_URLAUTH_NO_SCOPE: u32 = 20u32;
pub const MD_ERROR_SUB500_URLAUTH_NO_STORE: u32 = 17u32;
pub const MD_ERROR_SUB500_URLAUTH_STORE_ERROR: u32 = 18u32;
pub const MD_ERROR_SUB502_ARR_CONNECTION_ERROR: u32 = 3u32;
pub const MD_ERROR_SUB502_ARR_NO_SERVER: u32 = 4u32;
pub const MD_ERROR_SUB502_PREMATURE_EXIT: u32 = 2u32;
pub const MD_ERROR_SUB502_TIMEOUT: u32 = 1u32;
pub const MD_ERROR_SUB503_APP_CONCURRENT: u32 = 2u32;
pub const MD_ERROR_SUB503_ASPNET_QUEUE_FULL: u32 = 3u32;
pub const MD_ERROR_SUB503_CONNECTION_LIMIT: u32 = 5u32;
pub const MD_ERROR_SUB503_CPU_LIMIT: u32 = 1u32;
pub const MD_ERROR_SUB503_FASTCGI_QUEUE_FULL: u32 = 4u32;
pub const MD_EXIT_MESSAGE: u32 = 5001u32;
pub const MD_EXPORT_INHERITED: u32 = 1u32;
pub const MD_EXPORT_NODE_ONLY: u32 = 2u32;
pub const MD_EXTLOG_BYTES_RECV: u32 = 8192u32;
pub const MD_EXTLOG_BYTES_SENT: u32 = 4096u32;
pub const MD_EXTLOG_CLIENT_IP: u32 = 4u32;
pub const MD_EXTLOG_COMPUTER_NAME: u32 = 32u32;
pub const MD_EXTLOG_COOKIE: u32 = 131072u32;
pub const MD_EXTLOG_DATE: u32 = 1u32;
pub const MD_EXTLOG_HOST: u32 = 1048576u32;
pub const MD_EXTLOG_HTTP_STATUS: u32 = 1024u32;
pub const MD_EXTLOG_HTTP_SUB_STATUS: u32 = 2097152u32;
pub const MD_EXTLOG_METHOD: u32 = 128u32;
pub const MD_EXTLOG_PROTOCOL_VERSION: u32 = 524288u32;
pub const MD_EXTLOG_REFERER: u32 = 262144u32;
pub const MD_EXTLOG_SERVER_IP: u32 = 64u32;
pub const MD_EXTLOG_SERVER_PORT: u32 = 32768u32;
pub const MD_EXTLOG_SITE_NAME: u32 = 16u32;
pub const MD_EXTLOG_TIME: u32 = 2u32;
pub const MD_EXTLOG_TIME_TAKEN: u32 = 16384u32;
pub const MD_EXTLOG_URI_QUERY: u32 = 512u32;
pub const MD_EXTLOG_URI_STEM: u32 = 256u32;
pub const MD_EXTLOG_USERNAME: u32 = 8u32;
pub const MD_EXTLOG_USER_AGENT: u32 = 65536u32;
pub const MD_EXTLOG_WIN32_STATUS: u32 = 2048u32;
pub const MD_FILTER_DESCRIPTION: u32 = 2045u32;
pub const MD_FILTER_ENABLED: u32 = 2043u32;
pub const MD_FILTER_ENABLE_CACHE: u32 = 2046u32;
pub const MD_FILTER_FLAGS: u32 = 2044u32;
pub const MD_FILTER_IMAGE_PATH: u32 = 2041u32;
pub const MD_FILTER_LOAD_ORDER: u32 = 2040u32;
pub const MD_FILTER_STATE: u32 = 2042u32;
pub const MD_FILTER_STATE_LOADED: u32 = 1u32;
pub const MD_FILTER_STATE_UNLOADED: u32 = 4u32;
pub const MD_FOOTER_DOCUMENT: u32 = 6009u32;
pub const MD_FOOTER_ENABLED: u32 = 6010u32;
pub const MD_FRONTPAGE_WEB: u32 = 2072u32;
pub const MD_FTPS_128_BITS: u32 = 5053u32;
pub const MD_FTPS_ALLOW_CCC: u32 = 5054u32;
pub const MD_FTPS_SECURE_ANONYMOUS: u32 = 5052u32;
pub const MD_FTPS_SECURE_CONTROL_CHANNEL: u32 = 5050u32;
pub const MD_FTPS_SECURE_DATA_CHANNEL: u32 = 5051u32;
pub const MD_FTP_KEEP_PARTIAL_UPLOADS: u32 = 5019u32;
pub const MD_FTP_LOG_IN_UTF_8: u32 = 5013u32;
pub const MD_FTP_PASV_RESPONSE_IP: u32 = 5018u32;
pub const MD_FTP_UTF8_FILE_NAMES: u32 = 5020u32;
pub const MD_GLOBAL_BINARY_LOGGING_ENABLED: u32 = 4016u32;
pub const MD_GLOBAL_BINSCHEMATIMESTAMP: u32 = 9991u32;
pub const MD_GLOBAL_CHANGE_NUMBER: u32 = 9997u32;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MAJOR_VERSION_NUMBER: u32 = 9994u32;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MINOR_VERSION_NUMBER: u32 = 9993u32;
pub const MD_GLOBAL_LOG_IN_UTF_8: u32 = 9206u32;
pub const MD_GLOBAL_SESSIONKEY: u32 = 9999u32;
pub const MD_GLOBAL_STANDARD_APP_MODE_ENABLED: u32 = 9203u32;
pub const MD_GLOBAL_XMLSCHEMATIMESTAMP: u32 = 9992u32;
pub const MD_GREETING_MESSAGE: u32 = 5002u32;
pub const MD_HC_CACHE_CONTROL_HEADER: u32 = 2211u32;
pub const MD_HC_COMPRESSION_BUFFER_SIZE: u32 = 2223u32;
pub const MD_HC_COMPRESSION_DIRECTORY: u32 = 2210u32;
pub const MD_HC_COMPRESSION_DLL: u32 = 2237u32;
pub const MD_HC_CREATE_FLAGS: u32 = 2243u32;
pub const MD_HC_DO_DISK_SPACE_LIMITING: u32 = 2216u32;
pub const MD_HC_DO_DYNAMIC_COMPRESSION: u32 = 2213u32;
pub const MD_HC_DO_NAMESPACE_DYNAMIC_COMPRESSION: u32 = 2255u32;
pub const MD_HC_DO_NAMESPACE_STATIC_COMPRESSION: u32 = 2256u32;
pub const MD_HC_DO_ON_DEMAND_COMPRESSION: u32 = 2215u32;
pub const MD_HC_DO_STATIC_COMPRESSION: u32 = 2214u32;
pub const MD_HC_DYNAMIC_COMPRESSION_LEVEL: u32 = 2241u32;
pub const MD_HC_EXPIRES_HEADER: u32 = 2212u32;
pub const MD_HC_FILES_DELETED_PER_DISK_FREE: u32 = 2225u32;
pub const MD_HC_FILE_EXTENSIONS: u32 = 2238u32;
pub const MD_HC_IO_BUFFER_SIZE: u32 = 2222u32;
pub const MD_HC_MAX_DISK_SPACE_USAGE: u32 = 2221u32;
pub const MD_HC_MAX_QUEUE_LENGTH: u32 = 2224u32;
pub const MD_HC_MIME_TYPE: u32 = 2239u32;
pub const MD_HC_MIN_FILE_SIZE_FOR_COMP: u32 = 2226u32;
pub const MD_HC_NO_COMPRESSION_FOR_HTTP_10: u32 = 2217u32;
pub const MD_HC_NO_COMPRESSION_FOR_PROXIES: u32 = 2218u32;
pub const MD_HC_NO_COMPRESSION_FOR_RANGE: u32 = 2219u32;
pub const MD_HC_ON_DEMAND_COMP_LEVEL: u32 = 2242u32;
pub const MD_HC_PRIORITY: u32 = 2240u32;
pub const MD_HC_SCRIPT_FILE_EXTENSIONS: u32 = 2244u32;
pub const MD_HC_SEND_CACHE_HEADERS: u32 = 2220u32;
pub const MD_HEADER_WAIT_TIMEOUT: u32 = 9204u32;
pub const MD_HISTORY_LATEST: u32 = 1u32;
pub const MD_HTTPERRORS_EXISTING_RESPONSE: u32 = 6056u32;
pub const MD_HTTP_CUSTOM: u32 = 6004u32;
pub const MD_HTTP_EXPIRES: u32 = 6002u32;
pub const MD_HTTP_FORWARDER_CUSTOM: u32 = 6054u32;
pub const MD_HTTP_PICS: u32 = 6003u32;
pub const MD_HTTP_REDIRECT: u32 = 6011u32;
pub const MD_IISADMIN_EXTENSIONS: u32 = 1028u32;
pub const MD_IMPORT_INHERITED: u32 = 1u32;
pub const MD_IMPORT_MERGE: u32 = 4u32;
pub const MD_IMPORT_NODE_ONLY: u32 = 2u32;
pub const MD_IN_PROCESS_ISAPI_APPS: u32 = 2073u32;
pub const MD_IP_SEC: u32 = 6019u32;
pub const MD_ISAPI_RESTRICTION_LIST: u32 = 2163u32;
pub const MD_IS_CONTENT_INDEXED: u32 = 6039u32;
pub const MD_KEY_TYPE: u32 = 1002u32;
pub const MD_LEVELS_TO_SCAN: u32 = 1022u32;
pub const MD_LOAD_BALANCER_CAPABILITIES: u32 = 9034u32;
pub const MD_LOAD_BALANCER_CAPABILITIES_BASIC: u32 = 1u32;
pub const MD_LOAD_BALANCER_CAPABILITIES_SOPHISTICATED: u32 = 2u32;
pub const MD_LOCATION: u32 = 9989u32;
pub const MD_LOGCUSTOM_DATATYPE_DOUBLE: u32 = 5u32;
pub const MD_LOGCUSTOM_DATATYPE_FLOAT: u32 = 4u32;
pub const MD_LOGCUSTOM_DATATYPE_INT: u32 = 0u32;
pub const MD_LOGCUSTOM_DATATYPE_LONG: u32 = 2u32;
pub const MD_LOGCUSTOM_DATATYPE_LPSTR: u32 = 6u32;
pub const MD_LOGCUSTOM_DATATYPE_LPWSTR: u32 = 7u32;
pub const MD_LOGCUSTOM_DATATYPE_UINT: u32 = 1u32;
pub const MD_LOGCUSTOM_DATATYPE_ULONG: u32 = 3u32;
pub const MD_LOGCUSTOM_PROPERTY_DATATYPE: u32 = 4505u32;
pub const MD_LOGCUSTOM_PROPERTY_HEADER: u32 = 4502u32;
pub const MD_LOGCUSTOM_PROPERTY_ID: u32 = 4503u32;
pub const MD_LOGCUSTOM_PROPERTY_MASK: u32 = 4504u32;
pub const MD_LOGCUSTOM_PROPERTY_NAME: u32 = 4501u32;
pub const MD_LOGCUSTOM_PROPERTY_NODE_ID: u32 = 4508u32;
pub const MD_LOGCUSTOM_SERVICES_STRING: u32 = 4506u32;
pub const MD_LOGEXT_FIELD_MASK: u32 = 4013u32;
pub const MD_LOGEXT_FIELD_MASK2: u32 = 4014u32;
pub const MD_LOGFILE_DIRECTORY: u32 = 4001u32;
pub const MD_LOGFILE_LOCALTIME_ROLLOVER: u32 = 4015u32;
pub const MD_LOGFILE_PERIOD: u32 = 4003u32;
pub const MD_LOGFILE_PERIOD_DAILY: u32 = 1u32;
pub const MD_LOGFILE_PERIOD_HOURLY: u32 = 4u32;
pub const MD_LOGFILE_PERIOD_MAXSIZE: u32 = 0u32;
pub const MD_LOGFILE_PERIOD_MONTHLY: u32 = 3u32;
pub const MD_LOGFILE_PERIOD_NONE: u32 = 0u32;
pub const MD_LOGFILE_PERIOD_WEEKLY: u32 = 2u32;
pub const MD_LOGFILE_TRUNCATE_SIZE: u32 = 4004u32;
pub const MD_LOGON_BATCH: u32 = 1u32;
pub const MD_LOGON_INTERACTIVE: u32 = 0u32;
pub const MD_LOGON_METHOD: u32 = 6013u32;
pub const MD_LOGON_NETWORK: u32 = 2u32;
pub const MD_LOGON_NETWORK_CLEARTEXT: u32 = 3u32;
pub const MD_LOGSQL_DATA_SOURCES: u32 = 4007u32;
pub const MD_LOGSQL_PASSWORD: u32 = 4010u32;
pub const MD_LOGSQL_TABLE_NAME: u32 = 4008u32;
pub const MD_LOGSQL_USER_NAME: u32 = 4009u32;
pub const MD_LOG_ANONYMOUS: u32 = 5007u32;
pub const MD_LOG_NONANONYMOUS: u32 = 5008u32;
pub const MD_LOG_PLUGINS_AVAILABLE: u32 = 4012u32;
pub const MD_LOG_PLUGIN_MOD_ID: u32 = 4005u32;
pub const MD_LOG_PLUGIN_ORDER: u32 = 4011u32;
pub const MD_LOG_PLUGIN_UI_ID: u32 = 4006u32;
pub const MD_LOG_TYPE: u32 = 4000u32;
pub const MD_LOG_TYPE_DISABLED: u32 = 0u32;
pub const MD_LOG_TYPE_ENABLED: u32 = 1u32;
pub const MD_LOG_UNUSED1: u32 = 4002u32;
pub const MD_MAX_BANDWIDTH: u32 = 1000u32;
pub const MD_MAX_BANDWIDTH_BLOCKED: u32 = 1003u32;
pub const MD_MAX_CHANGE_ENTRIES: u32 = 100u32;
pub const MD_MAX_CLIENTS_MESSAGE: u32 = 5003u32;
pub const MD_MAX_CONNECTIONS: u32 = 1014u32;
pub const MD_MAX_ENDPOINT_CONNECTIONS: u32 = 1024u32;
pub const MD_MAX_ERROR_FILES: u32 = 9988u32;
pub const MD_MAX_GLOBAL_BANDWIDTH: u32 = 9201u32;
pub const MD_MAX_GLOBAL_CONNECTIONS: u32 = 9202u32;
pub const MD_MAX_REQUEST_ENTITY_ALLOWED: u32 = 6051u32;
pub const MD_MD_SERVER_SS_AUTH_MAPPING: u32 = 2200u32;
pub const MD_METADATA_ID_REGISTRATION: u32 = 1030u32;
pub const MD_MIME_MAP: u32 = 6015u32;
pub const MD_MIN_FILE_BYTES_PER_SEC: u32 = 9205u32;
pub const MD_MSDOS_DIR_OUTPUT: u32 = 5004u32;
pub const MD_NETLOGON_WKS_DNS: u32 = 2u32;
pub const MD_NETLOGON_WKS_IP: u32 = 1u32;
pub const MD_NETLOGON_WKS_NONE: u32 = 0u32;
pub const MD_NET_LOGON_WKS: u32 = 2065u32;
pub const MD_NOTIFEXAUTH_NTLMSSL: u32 = 1u32;
pub const MD_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
pub const MD_NOTIFY_AUTHENTICATION: u32 = 8192u32;
pub const MD_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
pub const MD_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
pub const MD_NOTIFY_END_OF_REQUEST: u32 = 128u32;
pub const MD_NOTIFY_LOG: u32 = 512u32;
pub const MD_NOTIFY_NONSECURE_PORT: u32 = 2u32;
pub const MD_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
pub const MD_NOTIFY_ORDER_HIGH: u32 = 524288u32;
pub const MD_NOTIFY_ORDER_LOW: u32 = 131072u32;
pub const MD_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
pub const MD_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
pub const MD_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
pub const MD_NOTIFY_SECURE_PORT: u32 = 1u32;
pub const MD_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
pub const MD_NOTIFY_SEND_RESPONSE: u32 = 64u32;
pub const MD_NOTIFY_URL_MAP: u32 = 4096u32;
pub const MD_NOT_DELETABLE: u32 = 2116u32;
pub const MD_NTAUTHENTICATION_PROVIDERS: u32 = 6032u32;
pub const MD_PASSIVE_PORT_RANGE: u32 = 5016u32;
pub const MD_PASSPORT_NEED_MAPPING: u32 = 2u32;
pub const MD_PASSPORT_NO_MAPPING: u32 = 0u32;
pub const MD_PASSPORT_REQUIRE_AD_MAPPING: u32 = 6052u32;
pub const MD_PASSPORT_TRY_MAPPING: u32 = 1u32;
pub const MD_POOL_IDC_TIMEOUT: u32 = 6037u32;
pub const MD_PROCESS_NTCR_IF_LOGGED_ON: u32 = 2070u32;
pub const MD_PUT_READ_SIZE: u32 = 6046u32;
pub const MD_RAPID_FAIL_PROTECTION_INTERVAL: u32 = 9029u32;
pub const MD_RAPID_FAIL_PROTECTION_MAX_CRASHES: u32 = 9030u32;
pub const MD_REALM: u32 = 6001u32;
pub const MD_REDIRECT_HEADERS: u32 = 6044u32;
pub const MD_RESTRICTION_LIST_CUSTOM_DESC: u32 = 2165u32;
pub const MD_ROOT_ENABLE_EDIT_WHILE_RUNNING: u32 = 9998u32;
pub const MD_ROOT_ENABLE_HISTORY: u32 = 9996u32;
pub const MD_ROOT_MAX_HISTORY_FILES: u32 = 9995u32;
pub const MD_SCHEMA_METAID: u32 = 1004u32;
pub const MD_SCRIPTMAPFLAG_ALLOWED_ON_READ_DIR: u32 = 1u32;
pub const MD_SCRIPTMAPFLAG_CHECK_PATH_INFO: u32 = 4u32;
pub const MD_SCRIPTMAPFLAG_SCRIPT: u32 = 1u32;
pub const MD_SCRIPT_MAPS: u32 = 6014u32;
pub const MD_SCRIPT_TIMEOUT: u32 = 6033u32;
pub const MD_SECURE_BINDINGS: u32 = 2021u32;
pub const MD_SECURITY_SETUP_REQUIRED: u32 = 2166u32;
pub const MD_SERVER_AUTOSTART: u32 = 1017u32;
pub const MD_SERVER_BINDINGS: u32 = 1023u32;
pub const MD_SERVER_COMMAND: u32 = 1012u32;
pub const MD_SERVER_COMMAND_CONTINUE: u32 = 4u32;
pub const MD_SERVER_COMMAND_PAUSE: u32 = 3u32;
pub const MD_SERVER_COMMAND_START: u32 = 1u32;
pub const MD_SERVER_COMMAND_STOP: u32 = 2u32;
pub const MD_SERVER_COMMENT: u32 = 1015u32;
pub const MD_SERVER_CONFIGURATION_INFO: u32 = 1027u32;
pub const MD_SERVER_CONFIG_ALLOW_ENCRYPT: u32 = 4u32;
pub const MD_SERVER_CONFIG_AUTO_PW_SYNC: u32 = 8u32;
pub const MD_SERVER_CONFIG_SSL_128: u32 = 2u32;
pub const MD_SERVER_CONFIG_SSL_40: u32 = 1u32;
pub const MD_SERVER_LISTEN_BACKLOG: u32 = 1019u32;
pub const MD_SERVER_LISTEN_TIMEOUT: u32 = 1020u32;
pub const MD_SERVER_SIZE: u32 = 1018u32;
pub const MD_SERVER_SIZE_LARGE: u32 = 2u32;
pub const MD_SERVER_SIZE_MEDIUM: u32 = 1u32;
pub const MD_SERVER_SIZE_SMALL: u32 = 0u32;
pub const MD_SERVER_STATE: u32 = 1016u32;
pub const MD_SERVER_STATE_CONTINUING: u32 = 7u32;
pub const MD_SERVER_STATE_PAUSED: u32 = 6u32;
pub const MD_SERVER_STATE_PAUSING: u32 = 5u32;
pub const MD_SERVER_STATE_STARTED: u32 = 2u32;
pub const MD_SERVER_STATE_STARTING: u32 = 1u32;
pub const MD_SERVER_STATE_STOPPED: u32 = 4u32;
pub const MD_SERVER_STATE_STOPPING: u32 = 3u32;
pub const MD_SET_HOST_NAME: u32 = 2154u32;
pub const MD_SHOW_4_DIGIT_YEAR: u32 = 5010u32;
pub const MD_SSI_EXEC_DISABLED: u32 = 6028u32;
pub const MD_SSL_ACCESS_PERM: u32 = 6030u32;
pub const MD_SSL_ALWAYS_NEGO_CLIENT_CERT: u32 = 5521u32;
pub const MD_SSL_KEY_PASSWORD: u32 = 5502u32;
pub const MD_SSL_KEY_REQUEST: u32 = 5503u32;
pub const MD_SSL_PRIVATE_KEY: u32 = 5501u32;
pub const MD_SSL_PUBLIC_KEY: u32 = 5500u32;
pub const MD_SSL_USE_DS_MAPPER: u32 = 5519u32;
pub const MD_STOP_LISTENING: u32 = 9987u32;
pub const MD_SUPPRESS_DEFAULT_BANNER: u32 = 5017u32;
pub const MD_UPLOAD_READAHEAD_SIZE: u32 = 6045u32;
pub const MD_URL_AUTHORIZATION_IMPERSONATION_LEVEL: u32 = 6053u32;
pub const MD_URL_AUTHORIZATION_SCOPE_NAME: u32 = 6050u32;
pub const MD_URL_AUTHORIZATION_STORE_NAME: u32 = 6049u32;
pub const MD_USER_ISOLATION: u32 = 5012u32;
pub const MD_USER_ISOLATION_AD: u32 = 2u32;
pub const MD_USER_ISOLATION_BASIC: u32 = 1u32;
pub const MD_USER_ISOLATION_LAST: u32 = 2u32;
pub const MD_USER_ISOLATION_NONE: u32 = 0u32;
pub const MD_USE_DIGEST_SSP: u32 = 6047u32;
pub const MD_USE_HOST_NAME: u32 = 2066u32;
pub const MD_VR_IGNORE_TRANSLATE: u32 = 3008u32;
pub const MD_VR_NO_CACHE: u32 = 3007u32;
pub const MD_VR_PASSTHROUGH: u32 = 3006u32;
pub const MD_VR_PASSWORD: u32 = 3003u32;
pub const MD_VR_PATH: u32 = 3001u32;
pub const MD_VR_USERNAME: u32 = 3002u32;
pub const MD_WAM_PWD: u32 = 7502u32;
pub const MD_WAM_USER_NAME: u32 = 7501u32;
pub const MD_WARNING_DUP_NAME: i32 = 837636i32;
pub const MD_WARNING_INVALID_DATA: i32 = 837637i32;
pub const MD_WARNING_PATH_NOT_FOUND: i32 = 837635i32;
pub const MD_WARNING_PATH_NOT_INSERTED: i32 = 837639i32;
pub const MD_WARNING_SAVE_FAILED: i32 = 837641i32;
pub const MD_WEBDAV_MAX_ATTRIBUTES_PER_ELEMENT: u32 = 8501u32;
pub const MD_WEB_SVC_EXT_RESTRICTION_LIST: u32 = 2168u32;
pub const MD_WIN32_ERROR: u32 = 1099u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct METADATATYPES(pub i32);
pub const ALL_METADATA: METADATATYPES = METADATATYPES(0i32);
pub const DWORD_METADATA: METADATATYPES = METADATATYPES(1i32);
pub const STRING_METADATA: METADATATYPES = METADATATYPES(2i32);
pub const BINARY_METADATA: METADATATYPES = METADATATYPES(3i32);
pub const EXPANDSZ_METADATA: METADATATYPES = METADATATYPES(4i32);
pub const MULTISZ_METADATA: METADATATYPES = METADATATYPES(5i32);
pub const INVALID_END_METADATA: METADATATYPES = METADATATYPES(6i32);
impl ::std::convert::From<i32> for METADATATYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for METADATATYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const METADATA_DONT_EXPAND: u32 = 512u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct METADATA_GETALL_INTERNAL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub Anonymous: METADATA_GETALL_INTERNAL_RECORD_0,
    pub dwMDDataTag: u32,
}
impl METADATA_GETALL_INTERNAL_RECORD {}
impl ::std::default::Default for METADATA_GETALL_INTERNAL_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for METADATA_GETALL_INTERNAL_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for METADATA_GETALL_INTERNAL_RECORD {}
unsafe impl ::windows::runtime::Abi for METADATA_GETALL_INTERNAL_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union METADATA_GETALL_INTERNAL_RECORD_0 {
    pub dwMDDataOffset: usize,
    pub pbMDData: *mut u8,
}
impl METADATA_GETALL_INTERNAL_RECORD_0 {}
impl ::std::default::Default for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for METADATA_GETALL_INTERNAL_RECORD_0 {}
unsafe impl ::windows::runtime::Abi for METADATA_GETALL_INTERNAL_RECORD_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct METADATA_GETALL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub dwMDDataOffset: u32,
    pub dwMDDataTag: u32,
}
impl METADATA_GETALL_RECORD {}
impl ::std::default::Default for METADATA_GETALL_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for METADATA_GETALL_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("METADATA_GETALL_RECORD")
            .field("dwMDIdentifier", &self.dwMDIdentifier)
            .field("dwMDAttributes", &self.dwMDAttributes)
            .field("dwMDUserType", &self.dwMDUserType)
            .field("dwMDDataType", &self.dwMDDataType)
            .field("dwMDDataLen", &self.dwMDDataLen)
            .field("dwMDDataOffset", &self.dwMDDataOffset)
            .field("dwMDDataTag", &self.dwMDDataTag)
            .finish()
    }
}
impl ::std::cmp::PartialEq for METADATA_GETALL_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier
            && self.dwMDAttributes == other.dwMDAttributes
            && self.dwMDUserType == other.dwMDUserType
            && self.dwMDDataType == other.dwMDDataType
            && self.dwMDDataLen == other.dwMDDataLen
            && self.dwMDDataOffset == other.dwMDDataOffset
            && self.dwMDDataTag == other.dwMDDataTag
    }
}
impl ::std::cmp::Eq for METADATA_GETALL_RECORD {}
unsafe impl ::windows::runtime::Abi for METADATA_GETALL_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct METADATA_HANDLE_INFO {
    pub dwMDPermissions: u32,
    pub dwMDSystemChangeNumber: u32,
}
impl METADATA_HANDLE_INFO {}
impl ::std::default::Default for METADATA_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for METADATA_HANDLE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("METADATA_HANDLE_INFO")
            .field("dwMDPermissions", &self.dwMDPermissions)
            .field("dwMDSystemChangeNumber", &self.dwMDSystemChangeNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for METADATA_HANDLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDPermissions == other.dwMDPermissions
            && self.dwMDSystemChangeNumber == other.dwMDSystemChangeNumber
    }
}
impl ::std::cmp::Eq for METADATA_HANDLE_INFO {}
unsafe impl ::windows::runtime::Abi for METADATA_HANDLE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const METADATA_INHERIT: u32 = 1u32;
pub const METADATA_INSERT_PATH: u32 = 64u32;
pub const METADATA_ISINHERITED: u32 = 32u32;
pub const METADATA_LOCAL_MACHINE_ONLY: u32 = 128u32;
pub const METADATA_MASTER_ROOT_HANDLE: u32 = 0u32;
pub const METADATA_MAX_NAME_LEN: u32 = 256u32;
pub const METADATA_NON_SECURE_ONLY: u32 = 256u32;
pub const METADATA_NO_ATTRIBUTES: u32 = 0u32;
pub const METADATA_PARTIAL_PATH: u32 = 2u32;
pub const METADATA_PERMISSION_READ: u32 = 1u32;
pub const METADATA_PERMISSION_WRITE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct METADATA_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub pbMDData: *mut u8,
    pub dwMDDataTag: u32,
}
impl METADATA_RECORD {}
impl ::std::default::Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for METADATA_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("METADATA_RECORD")
            .field("dwMDIdentifier", &self.dwMDIdentifier)
            .field("dwMDAttributes", &self.dwMDAttributes)
            .field("dwMDUserType", &self.dwMDUserType)
            .field("dwMDDataType", &self.dwMDDataType)
            .field("dwMDDataLen", &self.dwMDDataLen)
            .field("pbMDData", &self.pbMDData)
            .field("dwMDDataTag", &self.dwMDDataTag)
            .finish()
    }
}
impl ::std::cmp::PartialEq for METADATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier
            && self.dwMDAttributes == other.dwMDAttributes
            && self.dwMDUserType == other.dwMDUserType
            && self.dwMDDataType == other.dwMDDataType
            && self.dwMDDataLen == other.dwMDDataLen
            && self.pbMDData == other.pbMDData
            && self.dwMDDataTag == other.dwMDDataTag
    }
}
impl ::std::cmp::Eq for METADATA_RECORD {}
unsafe impl ::windows::runtime::Abi for METADATA_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const METADATA_REFERENCE: u32 = 8u32;
pub const METADATA_SECURE: u32 = 4u32;
pub const METADATA_VOLATILE: u32 = 16u32;
pub const MSCS_MD_ID_BEGIN_RESERVED: u32 = 53248u32;
pub const MSCS_MD_ID_END_RESERVED: u32 = 57343u32;
pub const NNTP_MD_ID_BEGIN_RESERVED: u32 = 45056u32;
pub const NNTP_MD_ID_END_RESERVED: u32 = 49151u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PFN_GETEXTENSIONVERSION =
    unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK =
    unsafe extern "system" fn(
        pszurl: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK =
    unsafe extern "system" fn(
        pszprotocolmanagerdll: super::super::Foundation::PWSTR,
        pszprotocolmanagerdllinitfunction: super::super::Foundation::PWSTR,
        dwcustominterfaceid: u32,
        ppcustominterface: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PFN_HSE_IO_COMPLETION = unsafe extern "system" fn(
    pecb: *mut EXTENSION_CONTROL_BLOCK,
    pcontext: *mut ::std::ffi::c_void,
    cbio: u32,
    dwerror: u32,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PFN_HTTPEXTENSIONPROC =
    unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_TERMINATEEXTENSION =
    unsafe extern "system" fn(dwflags: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WEB_CORE_ACTIVATE = unsafe extern "system" fn(
    pszapphostconfigfile: super::super::Foundation::PWSTR,
    pszrootwebconfigfile: super::super::Foundation::PWSTR,
    pszinstancename: super::super::Foundation::PWSTR,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WEB_CORE_SET_METADATA_DLL_ENTRY =
    unsafe extern "system" fn(
        pszmetadatatype: super::super::Foundation::PWSTR,
        pszvalue: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT;
pub type PFN_WEB_CORE_SHUTDOWN =
    unsafe extern "system" fn(fimmediate: u32) -> ::windows::runtime::HRESULT;
pub const POP3_MD_ID_BEGIN_RESERVED: u32 = 40960u32;
pub const POP3_MD_ID_END_RESERVED: u32 = 45055u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct POST_PROCESS_PARAMETERS {
    pub pszSessionId: super::super::Foundation::PWSTR,
    pub pszSiteName: super::super::Foundation::PWSTR,
    pub pszUserName: super::super::Foundation::PWSTR,
    pub pszHostName: super::super::Foundation::PWSTR,
    pub pszRemoteIpAddress: super::super::Foundation::PWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: super::super::Foundation::PWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: super::super::Foundation::PWSTR,
    pub pszCommandParameters: super::super::Foundation::PWSTR,
    pub pszFullPath: super::super::Foundation::PWSTR,
    pub pszPhysicalPath: super::super::Foundation::PWSTR,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: ::windows::runtime::HRESULT,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl POST_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for POST_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for POST_PROCESS_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POST_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for POST_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.SessionStartTime == other.SessionStartTime
            && self.BytesSentPerSession == other.BytesSentPerSession
            && self.BytesReceivedPerSession == other.BytesReceivedPerSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for POST_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for POST_PROCESS_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRE_PROCESS_PARAMETERS {
    pub pszSessionId: super::super::Foundation::PWSTR,
    pub pszSiteName: super::super::Foundation::PWSTR,
    pub pszUserName: super::super::Foundation::PWSTR,
    pub pszHostName: super::super::Foundation::PWSTR,
    pub pszRemoteIpAddress: super::super::Foundation::PWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: super::super::Foundation::PWSTR,
    pub dwLocalIpPort: u32,
    pub pszCommand: super::super::Foundation::PWSTR,
    pub pszCommandParameters: super::super::Foundation::PWSTR,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl PRE_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRE_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRE_PROCESS_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRE_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRE_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.SessionStartTime == other.SessionStartTime
            && self.BytesSentPerSession == other.BytesSentPerSession
            && self.BytesReceivedPerSession == other.BytesReceivedPerSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRE_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRE_PROCESS_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SF_DENIED_APPLICATION: u32 = 8u32;
pub const SF_DENIED_BY_CONFIG: u32 = 65536u32;
pub const SF_DENIED_FILTER: u32 = 4u32;
pub const SF_DENIED_LOGON: u32 = 1u32;
pub const SF_DENIED_RESOURCE: u32 = 2u32;
pub const SF_MAX_AUTH_TYPE: u32 = 33u32;
pub const SF_MAX_FILTER_DESC_LEN: u32 = 257u32;
pub const SF_MAX_PASSWORD: u32 = 257u32;
pub const SF_MAX_USERNAME: u32 = 257u32;
pub const SF_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
pub const SF_NOTIFY_AUTHENTICATION: u32 = 8192u32;
pub const SF_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
pub const SF_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
pub const SF_NOTIFY_END_OF_REQUEST: u32 = 128u32;
pub const SF_NOTIFY_LOG: u32 = 512u32;
pub const SF_NOTIFY_NONSECURE_PORT: u32 = 2u32;
pub const SF_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
pub const SF_NOTIFY_ORDER_HIGH: u32 = 524288u32;
pub const SF_NOTIFY_ORDER_LOW: u32 = 131072u32;
pub const SF_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
pub const SF_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
pub const SF_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
pub const SF_NOTIFY_SECURE_PORT: u32 = 1u32;
pub const SF_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
pub const SF_NOTIFY_SEND_RESPONSE: u32 = 64u32;
pub const SF_NOTIFY_URL_MAP: u32 = 4096u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SF_PROPERTY_IIS(pub i32);
pub const SF_PROPERTY_SSL_CTXT: SF_PROPERTY_IIS = SF_PROPERTY_IIS(0i32);
pub const SF_PROPERTY_INSTANCE_NUM_ID: SF_PROPERTY_IIS = SF_PROPERTY_IIS(1i32);
impl ::std::convert::From<i32> for SF_PROPERTY_IIS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SF_PROPERTY_IIS {
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
pub struct SF_REQ_TYPE(pub i32);
pub const SF_REQ_SEND_RESPONSE_HEADER: SF_REQ_TYPE = SF_REQ_TYPE(0i32);
pub const SF_REQ_ADD_HEADERS_ON_DENIAL: SF_REQ_TYPE = SF_REQ_TYPE(1i32);
pub const SF_REQ_SET_NEXT_READ_SIZE: SF_REQ_TYPE = SF_REQ_TYPE(2i32);
pub const SF_REQ_SET_PROXY_INFO: SF_REQ_TYPE = SF_REQ_TYPE(3i32);
pub const SF_REQ_GET_CONNID: SF_REQ_TYPE = SF_REQ_TYPE(4i32);
pub const SF_REQ_SET_CERTIFICATE_INFO: SF_REQ_TYPE = SF_REQ_TYPE(5i32);
pub const SF_REQ_GET_PROPERTY: SF_REQ_TYPE = SF_REQ_TYPE(6i32);
pub const SF_REQ_NORMALIZE_URL: SF_REQ_TYPE = SF_REQ_TYPE(7i32);
pub const SF_REQ_DISABLE_NOTIFICATIONS: SF_REQ_TYPE = SF_REQ_TYPE(8i32);
impl ::std::convert::From<i32> for SF_REQ_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SF_REQ_TYPE {
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
pub struct SF_STATUS_TYPE(pub i32);
pub const SF_STATUS_REQ_FINISHED: SF_STATUS_TYPE = SF_STATUS_TYPE(134217728i32);
pub const SF_STATUS_REQ_FINISHED_KEEP_CONN: SF_STATUS_TYPE = SF_STATUS_TYPE(134217729i32);
pub const SF_STATUS_REQ_NEXT_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217730i32);
pub const SF_STATUS_REQ_HANDLED_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217731i32);
pub const SF_STATUS_REQ_ERROR: SF_STATUS_TYPE = SF_STATUS_TYPE(134217732i32);
pub const SF_STATUS_REQ_READ_NEXT: SF_STATUS_TYPE = SF_STATUS_TYPE(134217733i32);
impl ::std::convert::From<i32> for SF_STATUS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SF_STATUS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SMTP_MD_ID_BEGIN_RESERVED: u32 = 36864u32;
pub const SMTP_MD_ID_END_RESERVED: u32 = 40959u32;
pub const USER_MD_ID_BASE_RESERVED: u32 = 65535u32;
pub const WAM_MD_ID_BEGIN_RESERVED: u32 = 29952u32;
pub const WAM_MD_ID_END_RESERVED: u32 = 32767u32;
pub const WAM_MD_SERVER_BASE: u32 = 7500u32;
pub const WEBDAV_MD_SERVER_BASE: u32 = 8500u32;
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct _IIS_CRYPTO_BLOB(pub u8);
