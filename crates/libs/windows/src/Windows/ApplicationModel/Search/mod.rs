#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPane(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPane {
    type Vtable = ISearchPaneVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdacec38_3700_4d73_91a1_2f998674238a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(::windows::core::IUnknown);
impl ISearchPaneQueryChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchPaneQueryChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3c064fe9-2351-4248-a529-7110f464a785}");
}
unsafe impl ::windows::core::Interface for ISearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c064fe9_2351_4248_a529_7110f464a785);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82fb460e_0940_4b6d_b8d0_642b30989e15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    type Vtable = ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x460c92e5_4c32_4538_a4d4_b6b4400d140f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneStatics {
    type Vtable = ISearchPaneStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9572adf1_8f1d_481f_a15b_c61655f16a0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneStaticsWithHideThisApplication {
    type Vtable = ISearchPaneStaticsWithHideThisApplicationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00732830_50f1_4d03_99ac_c6644c8ed8b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplicationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81b10b1c_e561_4093_9b4d_2ad482794a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchQueryLinguisticDetailsFactory {
    type Vtable = ISearchQueryLinguisticDetailsFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcac6c3b8_3c64_4dfd_ad9f_479e4d4065a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytextalternatives: ::windows::core::RawPtr, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suggestions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, image: ::windows::core::RawPtr, imagealternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71598a9_c065_456d_a845_1eccec5dc28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(::windows::core::IUnknown);
impl LocalContentSuggestionSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalContentSuggestionSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>>(result__)
        }
    }
    pub fn SetAqsFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AqsFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PropertiesToMatch(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalContentSuggestionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalContentSuggestionSettings {}
unsafe impl ::windows::core::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.LocalContentSuggestionSettings;{eeaeb062-743d-456e-84a3-23f06f2d15d7})");
}
unsafe impl ::windows::core::Interface for LocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
}
impl ::windows::core::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
impl ::core::convert::From<LocalContentSuggestionSettings> for ::windows::core::IUnknown {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalContentSuggestionSettings> for ::windows::core::IUnknown {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalContentSuggestionSettings> for ::windows::core::IInspectable {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalContentSuggestionSettings> for ::windows::core::IInspectable {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct SearchPane(::windows::core::IUnknown);
impl SearchPane {
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryContext(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPlaceholderText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn QueryChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQueryChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SuggestionsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuggestionsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn QuerySubmitted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuerySubmitted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ResultSuggestionChosen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResultSuggestionChosen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLocalContentSuggestionSettings<'a, Param0: ::windows::core::IntoParam<'a, LocalContentSuggestionSettings>>(&self, settings: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadWithQuery<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, query: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), query.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowOnKeyboardInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TrySetQueryText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, query: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), query.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPane>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn HideThisApplication() -> ::windows::core::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SearchPane, ISearchPaneStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SearchPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPane {}
unsafe impl ::windows::core::RuntimeType for SearchPane {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPane;{fdacec38-3700-4d73-91a1-2f998674238a})");
}
unsafe impl ::windows::core::Interface for SearchPane {
    type Vtable = ISearchPaneVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdacec38_3700_4d73_91a1_2f998674238a);
}
impl ::windows::core::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
impl ::core::convert::From<SearchPane> for ::windows::core::IUnknown {
    fn from(value: SearchPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPane> for ::windows::core::IUnknown {
    fn from(value: &SearchPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPane> for ::windows::core::IInspectable {
    fn from(value: SearchPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPane> for ::windows::core::IInspectable {
    fn from(value: &SearchPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(::windows::core::IUnknown);
impl SearchPaneQueryChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQueryChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs;{3c064fe9-2351-4248-a529-7110f464a785})");
}
unsafe impl ::windows::core::Interface for SearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c064fe9_2351_4248_a529_7110f464a785);
}
impl ::windows::core::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
impl ::core::convert::From<SearchPaneQueryChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneQueryChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchPaneQueryChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneQueryChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::core::convert::TryInto::<ISearchPaneQueryChangedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchPaneQueryChangedEventArgs {}
unsafe impl ::core::marker::Sync for SearchPaneQueryChangedEventArgs {}
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(::windows::core::IUnknown);
impl SearchPaneQueryLinguisticDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQueryLinguisticDetails {}
unsafe impl ::windows::core::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails;{82fb460e-0940-4b6d-b8d0-642b30989e15})");
}
unsafe impl ::windows::core::Interface for SearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82fb460e_0940_4b6d_b8d0_642b30989e15);
}
impl ::windows::core::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
impl ::core::convert::From<SearchPaneQueryLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneQueryLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchPaneQueryLinguisticDetails {}
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(::windows::core::IUnknown);
impl SearchPaneQuerySubmittedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneQuerySubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQuerySubmittedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs;{143ba4fc-e9c5-4736-91b2-e8eb9cb88356})");
}
unsafe impl ::windows::core::Interface for SearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
}
impl ::windows::core::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
impl ::core::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneQuerySubmittedEventArgs {}
unsafe impl ::core::marker::Sync for SearchPaneQuerySubmittedEventArgs {}
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(::windows::core::IUnknown);
impl SearchPaneResultSuggestionChosenEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneResultSuggestionChosenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneResultSuggestionChosenEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs;{c8316cc0-aed2-41e0-bce0-c26ca74f85ec})");
}
unsafe impl ::windows::core::Interface for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
}
impl ::windows::core::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
impl ::core::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneResultSuggestionChosenEventArgs {}
unsafe impl ::core::marker::Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(::windows::core::IUnknown);
impl SearchPaneSuggestionsRequest {
    #[cfg(feature = "deprecated")]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneSuggestionsRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneSuggestionsRequest {}
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest;{81b10b1c-e561-4093-9b4d-2ad482794a53})");
}
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81b10b1c_e561_4093_9b4d_2ad482794a53);
}
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
impl ::core::convert::From<SearchPaneSuggestionsRequest> for ::windows::core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequest> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneSuggestionsRequest> for ::windows::core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequest> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequest {}
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequest {}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(::windows::core::IUnknown);
impl SearchPaneSuggestionsRequestDeferral {
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestDeferral {}
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral;{a0d009f7-8748-4ee2-ad44-afa6be997c51})");
}
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
}
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
impl ::core::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestDeferral {}
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestDeferral {}
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(::windows::core::IUnknown);
impl SearchPaneSuggestionsRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Request(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneSuggestionsRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs;{c89b8a2f-ac56-4460-8d2f-80023bec4fc5})");
}
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
}
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
impl ::core::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchPaneSuggestionsRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneSuggestionsRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::core::convert::TryInto::<ISearchPaneQueryChangedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(::windows::core::IUnknown);
impl SearchPaneVisibilityChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneVisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneVisibilityChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs;{3c4d3046-ac4b-49f2-97d6-020e6182cb9c})");
}
unsafe impl ::windows::core::Interface for SearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
}
impl ::windows::core::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
impl ::core::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneVisibilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for SearchPaneVisibilityChangedEventArgs {}
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(::windows::core::IUnknown);
impl SearchQueryLinguisticDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(querytextalternatives: Param0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows::core::Result<SearchQueryLinguisticDetails> {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), querytextalternatives.into_param().abi(), querytextcompositionstart, querytextcompositionlength, &mut result__).from_abi::<SearchQueryLinguisticDetails>(result__)
        })
    }
    pub fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchQueryLinguisticDetails {}
unsafe impl ::windows::core::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchQueryLinguisticDetails;{46a1205b-69c9-4745-b72f-a8a4fc8f24ae})");
}
unsafe impl ::windows::core::Interface for SearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
}
impl ::windows::core::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
impl ::core::convert::From<SearchQueryLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchQueryLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchQueryLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchQueryLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchQueryLinguisticDetails {}
#[repr(transparent)]
pub struct SearchSuggestionCollection(::windows::core::IUnknown);
impl SearchSuggestionCollection {
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AppendQuerySuggestion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendQuerySuggestions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, suggestions: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), suggestions.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendResultSuggestion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, detailtext: Param1, tag: Param2, image: Param3, imagealternatetext: Param4) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), text.into_param().abi(), detailtext.into_param().abi(), tag.into_param().abi(), image.into_param().abi(), imagealternatetext.into_param().abi()).ok() }
    }
    pub fn AppendSearchSeparator<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, label: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), label.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionCollection {}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionCollection;{323a8a4b-fbea-4446-abbc-3da7915fdd3a})");
}
unsafe impl ::windows::core::Interface for SearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
}
impl ::windows::core::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
impl ::core::convert::From<SearchSuggestionCollection> for ::windows::core::IUnknown {
    fn from(value: SearchSuggestionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionCollection> for ::windows::core::IUnknown {
    fn from(value: &SearchSuggestionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchSuggestionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionCollection> for ::windows::core::IInspectable {
    fn from(value: SearchSuggestionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionCollection> for ::windows::core::IInspectable {
    fn from(value: &SearchSuggestionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchSuggestionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionCollection {}
unsafe impl ::core::marker::Sync for SearchSuggestionCollection {}
#[repr(transparent)]
pub struct SearchSuggestionsRequest(::windows::core::IUnknown);
impl SearchSuggestionsRequest {
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionsRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequest {}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequest;{4e4e26a7-44e5-4039-9099-6000ead1f0c6})");
}
unsafe impl ::windows::core::Interface for SearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
}
impl ::windows::core::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
impl ::core::convert::From<SearchSuggestionsRequest> for ::windows::core::IUnknown {
    fn from(value: SearchSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequest> for ::windows::core::IUnknown {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionsRequest> for ::windows::core::IInspectable {
    fn from(value: SearchSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequest> for ::windows::core::IInspectable {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionsRequest {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequest {}
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(::windows::core::IUnknown);
impl SearchSuggestionsRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequestDeferral {}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral;{b71598a9-c065-456d-a845-1eccec5dc28b})");
}
unsafe impl ::windows::core::Interface for SearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71598a9_c065_456d_a845_1eccec5dc28b);
}
impl ::windows::core::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
impl ::core::convert::From<SearchSuggestionsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionsRequestDeferral {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequestDeferral {}
