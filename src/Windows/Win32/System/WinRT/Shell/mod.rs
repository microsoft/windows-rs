#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Shell`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CreateProcessMethod(pub i32);
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
impl ::core::convert::From<i32> for CreateProcessMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CreateProcessMethod {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT_Shell`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDDEInitializer(pub ::windows::runtime::IUnknown);
impl IDDEInitializer {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    #[doc = "*Required features: `Win32_System_WinRT_Shell`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::super::UI::Shell::IShellItem>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
        Param8: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
    >(
        &self,
        fileextensionorprotocol: Param0,
        method: CreateProcessMethod,
        currentdirectory: Param2,
        exectarget: Param3,
        site: Param4,
        application: Param5,
        targetfile: Param6,
        arguments: Param7,
        verb: Param8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            fileextensionorprotocol.into_param().abi(),
            ::core::mem::transmute(method),
            currentdirectory.into_param().abi(),
            exectarget.into_param().abi(),
            site.into_param().abi(),
            application.into_param().abi(),
            targetfile.into_param().abi(),
            arguments.into_param().abi(),
            verb.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDDEInitializer {
    type Vtable = IDDEInitializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
}
impl ::core::convert::From<IDDEInitializer> for ::windows::runtime::IUnknown {
    fn from(value: IDDEInitializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDDEInitializer> for ::windows::runtime::IUnknown {
    fn from(value: &IDDEInitializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDDEInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDDEInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fileextensionorprotocol: super::super::super::Foundation::PWSTR,
        method: CreateProcessMethod,
        currentdirectory: super::super::super::Foundation::PWSTR,
        exectarget: ::windows::runtime::RawPtr,
        site: ::windows::runtime::RawPtr,
        application: super::super::super::Foundation::PWSTR,
        targetfile: super::super::super::Foundation::PWSTR,
        arguments: super::super::super::Foundation::PWSTR,
        verb: super::super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell")))] usize,
);
