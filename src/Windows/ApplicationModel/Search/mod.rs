#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4004425826, 29757, 17774, [132, 163, 35, 240, 111, 45, 21, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPane(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPane {
    type Vtable = ISearchPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4255968312, 14080, 19827, [145, 161, 47, 153, 134, 116, 35, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPane_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settings: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, query: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, query: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Search`*"]
pub struct ISearchPaneQueryChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007046633, 9041, 16968, [165, 41, 113, 16, 244, 100, 167, 133]);
}
impl ISearchPaneQueryChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3c064fe9-2351-4248-a529-7110f464a785}");
}
impl ::std::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2197505550, 2368, 19309, [184, 208, 100, 43, 48, 152, 158, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339453180, 59845, 18230, [145, 178, 232, 235, 156, 184, 131, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    type Vtable = ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1175229157, 19506, 17720, [164, 212, 182, 180, 64, 13, 20, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3358682304, 44754, 16864, [188, 224, 194, 108, 167, 79, 133, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneStatics {
    type Vtable = ISearchPaneStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2507320817, 36637, 18463, [161, 91, 198, 22, 85, 241, 106, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneStaticsWithHideThisApplication {
    type Vtable = ISearchPaneStaticsWithHideThisApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(7546928, 20721, 19715, [153, 172, 198, 100, 76, 142, 216, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2175863580, 58721, 16531, [155, 77, 42, 212, 130, 121, 74, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2697988599, 34632, 20194, [173, 68, 175, 166, 190, 153, 124, 81]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3365636655, 44118, 17504, [141, 47, 128, 2, 59, 236, 79, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1011691590, 44107, 18930, [151, 214, 2, 14, 97, 130, 203, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1184964699, 27081, 18245, [183, 47, 168, 164, 252, 143, 36, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchQueryLinguisticDetailsFactory {
    type Vtable = ISearchQueryLinguisticDetailsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3402023864, 15460, 19965, [173, 159, 71, 158, 77, 64, 101, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querytextalternatives: ::windows::runtime::RawPtr, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842697291, 64490, 17478, [171, 188, 61, 167, 145, 95, 221, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, suggestions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, detailtext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, image: ::windows::runtime::RawPtr, imagealternatetext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1313744551, 17637, 16441, [144, 153, 96, 0, 234, 209, 240, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3071645865, 49253, 17773, [168, 69, 30, 204, 236, 93, 194, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LocalContentSuggestionSettings(::windows::runtime::IInspectable);
impl LocalContentSuggestionSettings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LocalContentSuggestionSettings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`, `Storage`*"]
    pub fn Locations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetAqsFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn AqsFilter(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`*"]
    pub fn PropertiesToMatch(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.LocalContentSuggestionSettings;{eeaeb062-743d-456e-84a3-23f06f2d15d7})");
}
unsafe impl ::windows::runtime::Interface for LocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4004425826, 29757, 17774, [132, 163, 35, 240, 111, 45, 21, 215]);
}
impl ::windows::runtime::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
impl ::std::convert::From<LocalContentSuggestionSettings> for ::windows::runtime::IUnknown {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LocalContentSuggestionSettings> for ::windows::runtime::IUnknown {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LocalContentSuggestionSettings> for ::windows::runtime::IInspectable {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LocalContentSuggestionSettings> for ::windows::runtime::IInspectable {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct SearchContract(pub u8);
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPane(::windows::runtime::IInspectable);
impl SearchPane {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SearchHistoryEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetSearchHistoryContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SearchHistoryContext(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetPlaceholderText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn PlaceholderText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Visible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn VisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn QueryChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn RemoveQueryChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn SuggestionsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn RemoveSuggestionsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn QuerySubmitted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn RemoveQuerySubmitted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn ResultSuggestionChosen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation`*"]
    pub fn RemoveResultSuggestionChosen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetLocalContentSuggestionSettings<'a, Param0: ::windows::runtime::IntoParam<'a, LocalContentSuggestionSettings>>(&self, settings: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn ShowOverloadDefault(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn ShowOverloadWithQuery<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, query: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), query.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn ShowOnKeyboardInput(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn TrySetQueryText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, query: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), query.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPane>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn HideThisApplication() -> ::windows::runtime::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SearchPane, ISearchPaneStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPane {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPane;{fdacec38-3700-4d73-91a1-2f998674238a})");
}
unsafe impl ::windows::runtime::Interface for SearchPane {
    type Vtable = ISearchPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4255968312, 14080, 19827, [145, 161, 47, 153, 134, 116, 35, 138]);
}
impl ::windows::runtime::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
impl ::std::convert::From<SearchPane> for ::windows::runtime::IUnknown {
    fn from(value: SearchPane) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPane> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPane) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPane {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPane {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPane> for ::windows::runtime::IInspectable {
    fn from(value: SearchPane) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPane> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPane) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPane {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPane {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneQueryChangedEventArgs(::windows::runtime::IInspectable);
impl SearchPaneQueryChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs;{3c064fe9-2351-4248-a529-7110f464a785})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007046633, 9041, 16968, [165, 41, 113, 16, 244, 100, 167, 133]);
}
impl ::windows::runtime::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
impl ::std::convert::From<SearchPaneQueryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneQueryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISearchPaneQueryChangedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISearchPaneQueryChangedEventArgs>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SearchPaneQueryChangedEventArgs {}
unsafe impl ::std::marker::Sync for SearchPaneQueryChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneQueryLinguisticDetails(::windows::runtime::IInspectable);
impl SearchPaneQueryLinguisticDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`*"]
    pub fn QueryTextAlternatives(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryTextCompositionStart(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryTextCompositionLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails;{82fb460e-0940-4b6d-b8d0-642b30989e15})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2197505550, 2368, 19309, [184, 208, 100, 43, 48, 152, 158, 21]);
}
impl ::windows::runtime::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
impl ::std::convert::From<SearchPaneQueryLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneQueryLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneQueryLinguisticDetails {}
unsafe impl ::std::marker::Sync for SearchPaneQueryLinguisticDetails {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneQuerySubmittedEventArgs(::windows::runtime::IInspectable);
impl SearchPaneQuerySubmittedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::runtime::Interface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs;{143ba4fc-e9c5-4736-91b2-e8eb9cb88356})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339453180, 59845, 18230, [145, 178, 232, 235, 156, 184, 131, 86]);
}
impl ::windows::runtime::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
impl ::std::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneQuerySubmittedEventArgs {}
unsafe impl ::std::marker::Sync for SearchPaneQuerySubmittedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneResultSuggestionChosenEventArgs(::windows::runtime::IInspectable);
impl SearchPaneResultSuggestionChosenEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs;{c8316cc0-aed2-41e0-bce0-c26ca74f85ec})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3358682304, 44754, 16864, [188, 224, 194, 108, 167, 79, 133, 236]);
}
impl ::windows::runtime::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
impl ::std::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneResultSuggestionChosenEventArgs {}
unsafe impl ::std::marker::Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneSuggestionsRequest(::windows::runtime::IInspectable);
impl SearchPaneSuggestionsRequest {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SearchSuggestionCollection(&self) -> ::windows::runtime::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneSuggestionsRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest;{81b10b1c-e561-4093-9b4d-2ad482794a53})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2175863580, 58721, 16531, [155, 77, 42, 212, 130, 121, 74, 83]);
}
impl ::windows::runtime::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
impl ::std::convert::From<SearchPaneSuggestionsRequest> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneSuggestionsRequest> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneSuggestionsRequest {}
unsafe impl ::std::marker::Sync for SearchPaneSuggestionsRequest {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneSuggestionsRequestDeferral(::windows::runtime::IInspectable);
impl SearchPaneSuggestionsRequestDeferral {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral;{a0d009f7-8748-4ee2-ad44-afa6be997c51})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2697988599, 34632, 20194, [173, 68, 175, 166, 190, 153, 124, 81]);
}
impl ::windows::runtime::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
impl ::std::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneSuggestionsRequestDeferral {}
unsafe impl ::std::marker::Sync for SearchPaneSuggestionsRequestDeferral {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneSuggestionsRequestedEventArgs(::windows::runtime::IInspectable);
impl SearchPaneSuggestionsRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneSuggestionsRequest>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::runtime::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs;{c89b8a2f-ac56-4460-8d2f-80023bec4fc5})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3365636655, 44118, 17504, [141, 47, 128, 2, 59, 236, 79, 197]);
}
impl ::windows::runtime::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
impl ::std::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchPaneSuggestionsRequestedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchPaneSuggestionsRequestedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::std::convert::TryInto::<ISearchPaneQueryChangedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SearchPaneSuggestionsRequestedEventArgs {}
unsafe impl ::std::marker::Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchPaneVisibilityChangedEventArgs(::windows::runtime::IInspectable);
impl SearchPaneVisibilityChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Visible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs;{3c4d3046-ac4b-49f2-97d6-020e6182cb9c})");
}
unsafe impl ::windows::runtime::Interface for SearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1011691590, 44107, 18930, [151, 214, 2, 14, 97, 130, 203, 156]);
}
impl ::windows::runtime::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
impl ::std::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchPaneVisibilityChangedEventArgs {}
unsafe impl ::std::marker::Sync for SearchPaneVisibilityChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchQueryLinguisticDetails(::windows::runtime::IInspectable);
impl SearchQueryLinguisticDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`*"]
    pub fn QueryTextAlternatives(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryTextCompositionStart(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn QueryTextCompositionLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(querytextalternatives: Param0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows::runtime::Result<SearchQueryLinguisticDetails> {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), querytextalternatives.into_param().abi(), querytextcompositionstart, querytextcompositionlength, &mut result__).from_abi::<SearchQueryLinguisticDetails>(result__)
        })
    }
    pub fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchQueryLinguisticDetails;{46a1205b-69c9-4745-b72f-a8a4fc8f24ae})");
}
unsafe impl ::windows::runtime::Interface for SearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1184964699, 27081, 18245, [183, 47, 168, 164, 252, 143, 36, 174]);
}
impl ::windows::runtime::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
impl ::std::convert::From<SearchQueryLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchQueryLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchQueryLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchQueryLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchQueryLinguisticDetails {}
unsafe impl ::std::marker::Sync for SearchQueryLinguisticDetails {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchSuggestionCollection(::windows::runtime::IInspectable);
impl SearchSuggestionCollection {
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn AppendQuerySuggestion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Foundation_Collections`*"]
    pub fn AppendQuerySuggestions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, suggestions: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), suggestions.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Search`, `Storage_Streams`*"]
    pub fn AppendResultSuggestion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        text: Param0,
        detailtext: Param1,
        tag: Param2,
        image: Param3,
        imagealternatetext: Param4,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), text.into_param().abi(), detailtext.into_param().abi(), tag.into_param().abi(), image.into_param().abi(), imagealternatetext.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn AppendSearchSeparator<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, label: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), label.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionCollection;{323a8a4b-fbea-4446-abbc-3da7915fdd3a})");
}
unsafe impl ::windows::runtime::Interface for SearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842697291, 64490, 17478, [171, 188, 61, 167, 145, 95, 221, 58]);
}
impl ::windows::runtime::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
impl ::std::convert::From<SearchSuggestionCollection> for ::windows::runtime::IUnknown {
    fn from(value: SearchSuggestionCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchSuggestionCollection> for ::windows::runtime::IUnknown {
    fn from(value: &SearchSuggestionCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchSuggestionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchSuggestionCollection> for ::windows::runtime::IInspectable {
    fn from(value: SearchSuggestionCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchSuggestionCollection> for ::windows::runtime::IInspectable {
    fn from(value: &SearchSuggestionCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchSuggestionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchSuggestionCollection {}
unsafe impl ::std::marker::Sync for SearchSuggestionCollection {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchSuggestionsRequest(::windows::runtime::IInspectable);
impl SearchSuggestionsRequest {
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn SearchSuggestionCollection(&self) -> ::windows::runtime::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SearchSuggestionsRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequest;{4e4e26a7-44e5-4039-9099-6000ead1f0c6})");
}
unsafe impl ::windows::runtime::Interface for SearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1313744551, 17637, 16441, [144, 153, 96, 0, 234, 209, 240, 198]);
}
impl ::windows::runtime::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
impl ::std::convert::From<SearchSuggestionsRequest> for ::windows::runtime::IUnknown {
    fn from(value: SearchSuggestionsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchSuggestionsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchSuggestionsRequest> for ::windows::runtime::IInspectable {
    fn from(value: SearchSuggestionsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchSuggestionsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchSuggestionsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchSuggestionsRequest {}
unsafe impl ::std::marker::Sync for SearchSuggestionsRequest {}
#[doc = "*Required features: `ApplicationModel_Search`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SearchSuggestionsRequestDeferral(::windows::runtime::IInspectable);
impl SearchSuggestionsRequestDeferral {
    #[doc = "*Required features: `ApplicationModel_Search`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral;{b71598a9-c065-456d-a845-1eccec5dc28b})");
}
unsafe impl ::windows::runtime::Interface for SearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3071645865, 49253, 17773, [168, 69, 30, 204, 236, 93, 194, 139]);
}
impl ::windows::runtime::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
impl ::std::convert::From<SearchSuggestionsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SearchSuggestionsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SearchSuggestionsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SearchSuggestionsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SearchSuggestionsRequestDeferral {}
unsafe impl ::std::marker::Sync for SearchSuggestionsRequestDeferral {}
